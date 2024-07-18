use crate::{routes, AppState};
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(routes::get_documents))
        .route("/documents/:id/view", get(routes::view_document))
        .route("/documents/:id/dialogue",get(routes::add_to_repo_dialogue_document))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
