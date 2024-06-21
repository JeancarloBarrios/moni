use askama_axum::IntoResponse;

use crate::templates;

pub async fn home() -> impl IntoResponse {
    templates::Index
}
