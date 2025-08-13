use axum::Json;

pub async fn hello_world() -> Json<String> {
    Json(String::from("Hello world!"))
}
