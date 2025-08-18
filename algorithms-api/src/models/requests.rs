use serde::Deserialize;


#[derive(Deserialize)]
pub struct SortPayload {
    pub data: Vec<f64>
}