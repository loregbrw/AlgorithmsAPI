use crate::{algorithms::{self, sort::bubble_sort::BubbleSort, SortAlgorithm}, models::requests::SortPayload};

use axum::{extract::Path, http::StatusCode, Json};

pub enum AlgorithmType {
    BUBBLE
}



pub async fn hello_world() -> Json<String> {
    Json(String::from("Hello world!"))
}

pub async fn sort_handler(
    Path(algorithm): Path<String>,
    Json(request): Json<SortPayload>
) -> Result<Json<Vec<f64>>, StatusCode> {
    let algorithm: Box<dyn SortAlgorithm> = match algorithm.as_str() {
        "bubble" | "bubble-sort" => Box::new(BubbleSort),
        // ""
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(Json(algorithm.run(request.data)))
}
