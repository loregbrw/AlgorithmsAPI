use axum::{Router};
use tower::ServiceBuilder;

mod routes;
mod handlers;
mod algorithms;

const BIND_ADDR: &str = "127.0.0.1:2525";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::sort::router())
        .layer(
            ServiceBuilder::new()
                // .layer(layer)
        );

    let listener = tokio::net::TcpListener::bind(BIND_ADDR).await.unwrap();

    println!("Listening on http://{BIND_ADDR}");

    axum::serve(listener, app).await.unwrap();
}
