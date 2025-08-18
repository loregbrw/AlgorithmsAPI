use crate::models::stats::Stats;

use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub algorithm: T,
    pub stats: Stats
}

#[derive(Serialize)]
pub struct Algorithm<T> {
    pub result: T,
    pub name: &'static str,
    pub complexity: &'static str
}