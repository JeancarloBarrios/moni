use axum::{routing::get, Router};
use crate::routes;
use tower_http::services::ServeDir;
pub fn init_router() -> Router {
    Router::new()
        .route("/", get(routes::home))
        .route("/documents", get(routes::get_documents))
        .nest_service("/static", ServeDir::new("static"))


}


