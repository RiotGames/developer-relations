use crate::config::Configuration;
use askama_warp::Template;
use base64::prelude::*;
use log::info;
use serde_derive::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::{http, Filter, Rejection, Reply};

#[derive(Serialize, Deserialize, Clone, Debug)]
/// An OAuth request containing a code.
struct Request {
    /// The code that was given to us after the user authenticated with the
    /// provider.
    pub code: String,
}

/// The OAuth2 response returned from the authorization server.
#[derive(Clone, Serialize, Deserialize, Debug, Template)]
#[template(path = "oauth.html")]
struct Response {
    /// The OAuth2 access token.
    pub access_token: String,
    /// The  OAuth2 refresh token.
    pub refresh_token: String,
    /// The OAuth2 scope.
    pub scope: String,
    /// The OAuth2 ID token.
    pub id_token: String,
    /// The OAuth2 token type.
    pub token_type: String,
    /// The OAuth2 expiration time in seconds.
    pub expires_in: u32,
}

/// Handle the OAuth flow.
///
/// This function handles the  OAuth flow and returns the access token.
///
/// # Arguments
///
/// * `cfg` is the configuration for the OAuth flow.
///
/// # Returns
///
/// A filter that handles the OAuth flow and returns the access token.
pub fn handle(
    cfg: &Configuration,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cfg = cfg.clone();
    warp::get()
        .and(warp::path("oauth"))
        .and(warp::query::<Request>())
        .map(move |req: Request| {
            info!("✍️ handling oauth request");
            let code = req.clone().code;
            let form = [
                ("grant_type", "authorization_code"),
                ("code", code.as_str()),
                ("redirect_uri", &cfg.callback_url()),
            ];
            let auth = BASE64_STANDARD.encode(format!("{}:{}", cfg.client_id, cfg.client_secret));
            let body: Response = ureq::post(cfg.token_url().as_str())
                .set("Authorization", format!("Basic {auth}").as_str())
                .send_form(&form)
                .expect("error sending token request")
                .into_json()
                .expect("error parsing oauth response");
            info!("✍️ completed handling oauth request");
            http::Response::builder()
                .status(StatusCode::OK)
                .body(body.to_string())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use httpmock::prelude::*;

    struct ServiceMock {
        server: MockServer,
    }

    impl ServiceMock {
        fn new() -> Self {
            let server = MockServer::start();

            server.mock(|when, then| {
                when.method(POST)
                    .path("/token")
                    .x_www_form_urlencoded_key_exists("code");
                then.status(200)
                    .header("content-type", "application/json")
                    .body(r#"{"access_token": "xyz", "refresh_token": "abc", "scope": "def", "id_token": "ghi", "token_type": "jkl", "expires_in": 3600}"#);
            });

            server.mock(|when, then| {
                when.method(POST).path("/token");
                then.status(401)
                    .header("content-type", "application/json")
                    .body(r#"{}"#);
            });

            ServiceMock { server }
        }

        fn configuration(&self) -> config::Configuration {
            config::Configuration {
                server: config::Server {
                    host: "".to_string(),
                    port: 443,
                    tls: None,
                },
                api_token: "".to_string(),
                client_id: "".to_string(),
                client_secret: "".to_string(),
                provider_url: self.server.url("").to_string(),
                callback_host: "".to_string(),
                account_data_url: "".to_string(),
                champion_data_url: "".to_string(),
            }
        }
    }

    #[tokio::test]
    async fn handle_returns_expected_result() {
        let cfg = ServiceMock::new().configuration();
        let filter = handle(&cfg);
        let res = warp::test::request().path("/oauth?code=abc").reply(&filter);

        assert_eq!(res.await.status(), 200, "Should return 200");
    }

    #[tokio::test]
    async fn handle_returns_invalid_code() {
        let cfg = ServiceMock::new().configuration();
        let filter = handle(&cfg);
        let res = warp::test::request().path("/oauth").reply(&filter);

        assert_eq!(res.await.status(), 400, "Should return 400");
    }
}
