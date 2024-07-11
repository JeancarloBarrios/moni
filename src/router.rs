use crate::{routes, AppState};
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(routes::home))
        .route("/documents", get(routes::get_documents))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
