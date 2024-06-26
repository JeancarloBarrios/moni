use askama_axum::IntoResponse;

use crate::templates;
use crate::templates::DocumentTemplate;
use crate::templates::HtmlTemplate;
use agent::documents::read_documents;
pub async fn home() -> impl IntoResponse {
    templates::Index
}


//get documents handler
pub async fn get_documents() -> impl IntoResponse {
    let template = DocumentTemplate { docs: read_documents().await};
    // HtmlTemplate(template)
    template
}