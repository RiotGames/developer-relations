use super::HtmlTemplate;
use crate::config::Configuration;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;

/// `Response` is a struct that represents the server's response  to a request.
///
/// # Fields
///
/// * `sign_in_url`: The URL that the user should  be redirected to in order to sign in.
#[derive(Template, Clone)]
#[template(path = "default.html")]
pub struct Response {
    sign_in_url: String,
}

pub async fn handle(State(cfg): State<Configuration>) -> impl IntoResponse {
    let res = Response {
        sign_in_url: cfg.sign_in_url(),
    };

    HtmlTemplate(res)
}
