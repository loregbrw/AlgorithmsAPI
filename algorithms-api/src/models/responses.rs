use crate::models::stats::{Stats, SystemStats};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum BaseResponse<T> {
    WithStats(WithStatsResponse<T>),
    AlgorithmOnly(AlgorithmResponse<T>),
}

impl<T> BaseResponse<T> {
    pub fn from_algorithm_response(
        algorithm_response: AlgorithmResponse<T>,
        with_stats: bool,
    ) -> Self {
        if with_stats {
            BaseResponse::WithStats(WithStatsResponse {
                algorithm: algorithm_response,
                system_stats: SystemStats::collect(),
            })
        } else {
            BaseResponse::AlgorithmOnly(algorithm_response)
        }
    }
}

#[derive(Serialize)]
pub struct WithStatsResponse<T> {
    pub algorithm: AlgorithmResponse<T>,
    pub system_stats: SystemStats,
}

#[derive(Serialize)]
pub struct AlgorithmResponse<T> {
    pub result: T,
    pub name: &'static str,
    pub complexity: &'static str,
    pub stats: Stats,
}