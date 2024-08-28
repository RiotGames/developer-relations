use super::HtmlTemplate;
use crate::config::Configuration;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;

/// Represents the server's response to a request with a sign-in URL.
///
/// This struct is used to generate the HTML response for the client, directing them to the sign-in page.
/// It leverages the Askama template engine to render the `default.html` template with the provided `sign_in_url`.
#[derive(Template, Clone)]
#[template(path = "default.html")]
pub struct Response {
    /// The URL to which the user should be redirected for signing in.
    sign_in_url: String,
}

/// Handles requests by generating a response with a sign-in URL.
///
/// This asynchronous function is an Axum handler that constructs a `Response` struct with the sign-in URL
/// from the application configuration. It then wraps this `Response` in an `HtmlTemplate` to be rendered into HTML.
/// The resulting HTML is sent back to the client, directing them to the sign-in page.
///
/// # Arguments
/// * `cfg` - The application configuration, containing the sign-in URL.
///
/// # Returns
/// An implementation of `IntoResponse`, which Axum can convert into an HTTP response to be sent to the client.
pub async fn handle(State(cfg): State<Configuration>) -> impl IntoResponse {
    let res = Response {
        sign_in_url: cfg.sign_in_url(),
    };

    HtmlTemplate(res)
}
