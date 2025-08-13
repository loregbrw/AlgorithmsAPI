use crate::algorithms::sort::bubble_sort::BubbleSort;
use std::sync::Arc;
use axum::Json;

pub enum AlgorithmType {
    BUBBLE
}



pub async fn hello_world() -> Json<String> {
    Json(String::from("Hello world!"))
}

// pub async fn bubble_sort_handler() {
//     let algorithm = Arc::new(BubbleSort);
// }