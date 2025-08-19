use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParameters {
    #[serde(default, rename = "system-stats")]
    pub system_stats: bool,
}

#[derive(Deserialize)]
pub struct SortPayload {
    pub data: Vec<f64>,
}
