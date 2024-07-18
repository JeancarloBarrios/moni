use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use crate::documents::{Document, DocumentInsight, DocumentMessage};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

//a wrapper for turning askama templates into responses that can be handled by server
pub struct HtmlTemplate<T>(pub T);


#[derive(Template)]
#[template(path = "documents.html")]
pub struct DocumentsTemplate {
    pub docs: Vec<Document>,
}

#[derive(Template)]
#[template(path = "document_detail.html")]
pub struct DocumentDetailsTemplate {
    pub document: Document,
    pub document_chat: Vec<DocumentMessage>,
}

#[derive(Template)]
#[template(path = "add_to_report_dialogue.html")]
pub struct AddToReportDialogueTemplate {
    pub insight: DocumentInsight,
}