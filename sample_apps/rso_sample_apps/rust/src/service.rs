use crate::{config, handlers};
use futures::executor;
use log::debug;
use warp::Filter;

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
pub(crate) fn listen(cfg: &config::Configuration) {
    debug!("parsing server {}", cfg.server.addr());
    match cfg.server.addr().parse::<std::net::SocketAddr>() {
        Ok(addr) => {
            debug!("parsed server {}", addr);
            let routes = warp::get()
                .and(handlers::oauth::handle(cfg))
                .or(handlers::data::handle(cfg))
                .or(handlers::default::handle(cfg));
            match cfg.clone().server.tls {
                Some(tls) => {
                    executor::block_on(
                        warp::serve(routes)
                            .tls()
                            .cert_path(tls.cert)
                            .key_path(tls.key)
                            .run(addr),
                    );
                }
                None => {
                    executor::block_on(warp::serve(routes).run(addr));
                }
            }
        }
        Err(e) => {
            panic!("{e}");
        }
    }
}
