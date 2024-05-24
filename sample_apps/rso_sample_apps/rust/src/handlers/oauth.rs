use super::HtmlTemplate;
use crate::config::Configuration;
use askama_warp::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use base64::prelude::*;
use log::info;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
/// An OAuth request containing a code.
pub struct Request {
    /// The code that was given to us after the user authenticated with the
    /// provider.
    pub code: String,
}

/// The OAuth2 response returned from the authorization server.
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
    let auth = BASE64_STANDARD.encode(format!("{}:{}", cfg.client_id, cfg.client_secret));
    let res: Response = ureq::post(cfg.token_url().as_str())
        .set("Authorization", format!("Basic {auth}").as_str())
        .send_form(&form)
        .expect("error sending token request")
        .into_json()
        .expect("error parsing oauth response");
    info!("✍️ completed handling oauth request");

    HtmlTemplate(res)
}
