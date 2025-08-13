use axum::{routing::get, Router};

use crate::handlers::sort::hello_world;

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
}