use crate::documents::Document;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

#[derive(Template)]
#[template(path = "documents.html")]
pub struct DocumentTemplate {
    pub docs: Vec<Document>,
}

