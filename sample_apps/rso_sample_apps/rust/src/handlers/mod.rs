use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

pub mod data;
pub mod default;
pub mod oauth;

/// A wrapper struct for Askama templates to facilitate their conversion into Axum responses.
///
/// This struct takes a generic type `T` which must implement the `Template` trait from the Askama crate.
/// It provides a mechanism to convert a given template into an HTTP response that can be returned from an Axum handler.
struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    /// Converts the contained Askama template into an Axum response.
    ///
    /// This method attempts to render the template into HTML. If successful, it returns an HTTP response
    /// with the rendered HTML. If the rendering process fails, it returns an HTTP 500 Internal Server Error
    /// response with the error message.
    ///
    /// # Returns
    /// An `axum::response::Response` that can be directly returned from an Axum route handler.
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response(),
        }
    }
}
