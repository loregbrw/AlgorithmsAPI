use axum::{routing::{get, post}, Router};

use crate::handlers::sort::{hello_world, sort_handler};

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/{algorithm}", post(sort_handler))
}