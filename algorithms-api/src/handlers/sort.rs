use crate::{
    algorithms::{
        sort::{sort_algorithms_endpoints},
    },
    executor::executor::Executor,
    models::{
        requests::{QueryParameters, SortPayload},
        responses::BaseResponse,
        stats::SystemStats,
    },
};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

pub async fn hello_world() -> Json<SystemStats> {
    Json(SystemStats::collect())
}

pub async fn sort_handler(
    Path(algorithm): Path<String>,
    Query(params): Query<QueryParameters>,
    Json(request): Json<SortPayload>,
) -> Result<Json<BaseResponse<Vec<f64>>>, StatusCode> {

    let endpoints = sort_algorithms_endpoints();

    let factory = endpoints
        .get(algorithm.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let algorithm = factory();

    let algorithm_response = Executor::execute(algorithm, request.data);

    let response = BaseResponse::from_algorithm_response(algorithm_response, params.system_stats);

    Ok(Json(response))
}
