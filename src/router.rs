use axum::{routing::get, Router};

use crate::routes;

pub fn init_router() -> Router {
    Router::new().route("/", get(routes::home))
}
