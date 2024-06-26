use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use agent::documents::Document;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

//a wrapper for turning askama templates into responses that can be handled by server
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(), // If rendering is successful, return an HTML response
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR, // If rendering fails, return an internal server error
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "documents.html")]
pub struct DocumentTemplate {
    pub docs: Vec<Document>,
}