use askama_axum::IntoResponse;

use crate::documents::read_documents;
use crate::templates;
use crate::templates::DocumentTemplate;
pub async fn home() -> impl IntoResponse {
    templates::Index
}

//get documents handler
pub async fn get_documents() -> impl IntoResponse {
    DocumentTemplate {
        docs: read_documents().await,
    };
}

