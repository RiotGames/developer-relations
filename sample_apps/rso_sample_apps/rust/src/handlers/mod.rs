use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

pub mod data;
pub mod default;
pub mod oauth;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response(),
        }
    }
}
