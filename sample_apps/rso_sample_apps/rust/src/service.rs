use crate::config::Configuration;
use crate::{config, handlers};
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use log::{debug, info};

fn create_app(cfg: &Configuration) -> Router {
    Router::new()
        .route("/data", get(handlers::data::handle))
        .route("/oauth", get(handlers::oauth::handle))
        .route("/", get(handlers::default::handle))
        .with_state(cfg.clone())
}

/// Runs the web service
///
/// # Arguments
///
/// `cfg`: The configuration to use for the service.
///
/// # Returns
///
/// None
///
/// # Panics
///
/// Panics if the host address is invalid.
pub(crate) async fn listen(cfg: &config::Configuration) {
    match cfg.server.addr().parse::<std::net::SocketAddr>() {
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
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use mock::AuthProvider;
    use tower::ServiceExt;

    fn configuration(auth: &AuthProvider) -> Configuration {
        Configuration {
            server: config::Server {
                host: "".to_string(),
                port: 443,
                tls: None,
            },
            api_token: "".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            provider_url: auth.server.url("").to_string(),
            callback_host: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
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
