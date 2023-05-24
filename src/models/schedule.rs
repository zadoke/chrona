use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub station_id: u32,
    pub trains: Vec<Train>,
    pub station_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Train {
    pub train_passed: bool,
    pub time: String,
    pub date: String,
    pub destination_station_id: u32,
    pub origin_station_id: u32,
    pub train_number: u32,
    pub destination_station_name: String,
    pub origin_station_name: String,
    pub operator: String,
    pub service_type: String,
    pub info: String,
}