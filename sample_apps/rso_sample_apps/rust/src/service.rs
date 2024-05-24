use crate::{config, handlers};
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use log::{debug, info};

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
            let app = Router::new()
                .route("/data", get(handlers::data::handle))
                .route("/oauth", get(handlers::oauth::handle))
                .route("/", get(handlers::default::handle))
                .with_state(cfg.clone());

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