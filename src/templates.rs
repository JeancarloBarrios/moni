use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use crate::documents::{Document};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

//a wrapper for turning askama templates into responses that can be handled by server
pub struct HtmlTemplate<T>(pub T);


#[derive(Template)]
#[template(path = "documents.html")]
pub struct DocumentTemplate {
    pub docs: Vec<Document>,
}