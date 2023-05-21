use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainData {
    pub arrival_time: String,
    pub departure_time: String,
    pub destination: String,
    pub duration: String,
    pub stops: Vec<TrainStop>,
    pub operator: String,
    pub origin: String,
    pub status: String,
    pub service_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainStop {
    pub train_passed: bool,
    pub scheduled_time: String,
    pub node_id: i32,
    pub station_name: String,
    pub delay_info: String,
}
