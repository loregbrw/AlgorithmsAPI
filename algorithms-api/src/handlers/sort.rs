use crate::{
    algorithms::{self, sort::bubble_sort::BubbleSort, SortAlgorithm}, executor::executor::Executor, models::{
        requests::{QueryParameters, SortPayload}, responses::{AlgorithmResponse, BaseResponse, WithStatsResponse}, stats::SystemStats
    }
};
use axum::{extract::{Path, Query}, http::{response, StatusCode}, Json};

pub async fn hello_world() -> Json<SystemStats> {
    Json(SystemStats::collect())
}

pub async fn sort_handler(
    Path(algorithm): Path<String>,
    Query(params): Query<QueryParameters>,
    Json(request): Json<SortPayload>,
) -> Result<Json<BaseResponse<Vec<f64>>>, StatusCode> {
    let algorithm: Box<dyn SortAlgorithm> = match algorithm.as_str() {
        "bubble" | "bubble-sort" => Box::new(BubbleSort),
        // ""
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let algorithm_response = Executor::execute(algorithm, request.data);

    let response = if params.system_stats {
        BaseResponse::WithStats(WithStatsResponse {
            algorithm: algorithm_response,
            system_stats: SystemStats::collect()
        })
    } else {
        BaseResponse::AlgorithmOnly(algorithm_response)
    };

    Ok(Json(response))
}
