use crate::config::Configuration;
use crate::{config, handlers};
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use log::{debug, info};

/// Creates an instance of `Router` configured with routes and application state.
///
/// This function sets up the application's routes by associating URL paths with their respective handler functions.
/// It also attaches the application configuration to the state of the router, making it accessible to the handlers.
///
/// # Arguments
/// * `cfg` - A reference to the application's configuration.
///
/// # Returns
///
/// Returns an instance of `Router` configured with the application's routes and state.
fn create_app(cfg: &Configuration) -> Router {
    Router::new()
        .route("/data", get(handlers::data::handle))
        .route("/oauth", get(handlers::oauth::handle))
        .route("/", get(handlers::default::handle))
        .with_state(cfg.clone())
}

/// Starts the web service with the provided configuration.
///
/// This asynchronous function attempts to parse the server address from the configuration and start the server.
/// If TLS configuration is provided, it starts a TLS server; otherwise, it starts a regular HTTP server.
/// It logs the server's start-up and panics if the server address is invalid or if there are issues starting the server.
///
/// # Arguments
/// * `cfg` - A reference to the configuration to use for the service.
///
/// # Panics
///
/// Panics if the host address is invalid or if there are issues starting the server.
pub(crate) async fn listen(cfg: &config::Configuration) {
    match cfg.server.addr.parse::<std::net::SocketAddr>() {
        Ok(addr) => {
            let app = create_app(cfg);
            match cfg.clone().server.tls {
                Some(tls) => {
                    let config = RustlsConfig::from_pem_file(tls.cert, tls.key)
                        .await
                        .unwrap();
                    info!("☁️ starting server with tls @ {addr}");
                    axum_server::bind_rustls(addr, config)
                        .serve(app.into_make_service())
                        .await
                        .unwrap();
                }
                None => {
                    debug!("☁️ starting server @ {addr}");
                    axum_server::bind(addr)
                        .serve(app.into_make_service())
                        .await
                        .unwrap();
                }
            }
        }
        Err(e) => {
            panic!("{e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Api, Rso, Tls, Urls};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use mock::AuthProvider;
    use tower::ServiceExt;

    fn configuration(auth: &AuthProvider) -> Configuration {
        Configuration {
            server: crate::config::Server {
                addr: "0.0.0.0:443".to_string(),
                tls: Some(Tls {
                    cert: "cert".to_string(),
                    key: "key".to_string(),
                }),
            },
            api: Api {
                token: "token".to_string(),
                urls: Urls {
                    account_data: auth.server.url("/riot/account/v1/accounts/me"),
                    champion_data: auth.server.url("/lol/platform/v3/champion-rotations"),
                },
            },
            rso: Rso {
                base_url: auth.server.url("").to_string(),
                callback_host: "local.example.com:8080".to_string(),
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
            },
        }
    }
    #[tokio::test]
    async fn default() {
        let prov = mock::AuthProvider::new();
        let cfg = configuration(&prov);
        let app = create_app(&cfg);
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn oauth_no_code() {
        let prov = mock::AuthProvider::new();
        let cfg = configuration(&prov);
        let app = create_app(&cfg);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/oauth")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn oauth_code() {
        let prov = mock::AuthProvider::new();
        let cfg = configuration(&prov);
        let app = create_app(&cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/oauth?code=200")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn data_returns_expected_result() {
        let prov = mock::AuthProvider::new();
        let cfg = configuration(&prov);
        let app = create_app(&cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/data?access_token=200")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn data_returns_unauthorized_when_no_access_token() {
        let prov = mock::AuthProvider::new();
        let cfg = configuration(&prov);
        let app = create_app(&cfg);

        let response = app
            .oneshot(Request::builder().uri("/data").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
