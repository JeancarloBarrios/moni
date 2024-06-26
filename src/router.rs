use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::routes;

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(routes::home))
        .nest_service("/static", ServeDir::new("static"))
}
