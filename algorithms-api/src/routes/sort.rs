use crate::handlers::sort::{hello_world, sort_handler};
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/{algorithm}", post(sort_handler))
}
