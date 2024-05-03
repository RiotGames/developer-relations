use crate::config::Configuration;
use askama::Template;
use log::info;
use warp::{Filter, Rejection, Reply};

/// `Response` is a struct that represents the server's response  to a request.
///
/// # Fields
///
/// * `sign_in_url`: The URL that the user should  be redirected to in order to sign in.
#[derive(Template, Clone)]
#[template(path = "default.html")]
struct Response {
    sign_in_url: String,
}

/// Handles the root route by returning a `Response` containing the sign  in URL.
///
/// # Arguments
///
/// * `cfg`: The configuration for the application.
///
/// # Returns
///
/// An implementation of the `Filter` trait that can be used to handle the root route.
/// The filter will map any request to a `Response` containing the sign in URL.
///
/// # Example
///
/// ```
/// use warp::{Filter, Reply};
///
/// fn main () {
///     let cfg = Configuration::default();
///
///     let route = handle(cfg);
///
///     warp::serve(route).run(([0, 0, 0, 0], 8080));
/// }
/// ```
pub fn handle(
    cfg: &Configuration,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    info!("📄 default page requested");
    let cfg = cfg.clone();
    warp::get().map(move || Response {
        sign_in_url: cfg.sign_in_url(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handle_returns_expected_result() {
        let cfg = crate::config::Configuration {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            provider_url: "https://auth.riotgames.com".to_string(),
            server: crate::config::Server {
                host: "".to_string(),
                port: 0,
                tls: None,
            },
            callback_host: "".to_string(),
            api_token: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
        };
        let filter = handle(&cfg);
        let res = warp::test::request().path("/").reply(&filter);

        assert_eq!(res.await.status(), 200, "Should return 200");
    }
}
