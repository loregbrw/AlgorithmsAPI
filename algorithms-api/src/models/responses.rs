use crate::models::stats::{Stats, SystemStats};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum BaseResponse<T> {
    WithStats(WithStatsResponse<T>),
    AlgorithmOnly(AlgorithmResponse<T>),
}

#[derive(Serialize)]
pub struct WithStatsResponse<T> {
    pub algorithm: AlgorithmResponse<T>,
    pub system_stats: SystemStats
}

#[derive(Serialize)]
pub struct AlgorithmResponse<T> {
    pub result: T,
    pub name: &'static str,
    pub complexity: &'static str,
    pub stats: Stats
}