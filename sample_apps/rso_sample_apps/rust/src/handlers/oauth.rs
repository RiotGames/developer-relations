use super::HtmlTemplate;
use crate::config::Configuration;
use askama::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use base64::prelude::*;
use log::info;

use serde::{Deserialize, Serialize};

/// Represents an OAuth request containing a code.
///
/// This struct is used to deserialize the query parameters of an incoming OAuth request,
/// specifically capturing the authorization code provided by the OAuth provider after
/// user authentication.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Request {
    /// The authorization code provided by the OAuth provider.
    pub code: String,
}

/// Represents the OAuth2 response returned from the authorization server.
///
/// This struct is used to serialize the OAuth2 tokens and related information received
/// from the authorization server into a format that can be rendered into an HTML template.
/// It includes access and refresh tokens, scope, ID token, token type, and expiration time.
#[derive(Clone, Serialize, Deserialize, Debug, Template)]
#[template(path = "oauth.html")]
pub struct Response {
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

/// Handles incoming OAuth requests by exchanging the authorization code for tokens.
///
/// This asynchronous function acts as an Axum handler for OAuth requests. It extracts the
/// authorization code from the query parameters, constructs a request to the authorization
/// server to exchange the code for an access token and other tokens, and then renders the
/// response into an HTML template using the `HtmlTemplate` wrapper.
///
/// # Arguments
/// * `Query(query)` - The extracted query parameters containing the authorization code.
/// * `State(cfg)` - The application configuration state, containing OAuth client credentials
///   and endpoints.
///
/// # Returns
/// An implementation of `IntoResponse`, which can be converted into an HTTP response to be
/// sent back to the client. This response includes the OAuth tokens rendered into an HTML
/// template.
pub async fn handle(
    Query(query): Query<Request>,
    State(cfg): State<Configuration>,
) -> impl IntoResponse {
    info!("✍️ handling oauth request");
    let code = query.code;
    let form = [
        ("grant_type", "authorization_code"),
        ("code", code.as_str()),
        ("redirect_uri", &cfg.callback_url()),
    ];
    let auth = BASE64_STANDARD.encode(format!("{}:{}", cfg.rso.client_id, cfg.rso.client_secret));
    let res: Response = ureq::post(cfg.token_url().as_str())
        .set("Authorization", format!("Basic {auth}").as_str())
        .send_form(&form)
        .expect("error sending token request")
        .into_json()
        .expect("error parsing oauth response");
    info!("✍️ completed handling oauth request");

    HtmlTemplate(res)
}
