use axum::{extract::Path, response::IntoResponse, Json};
use serde_json::json;
use reqwest::Client;
use std::sync::Arc;

use crate::services::fetch_schedule_data;

pub async fn handler(client: Arc<Client>, Path(station_number): Path<i32>, schedule_type: usize) -> impl IntoResponse {
    match fetch_schedule_data(&client, &station_number, schedule_type).await {
        Ok(schedule_data) => Json(schedule_data),
        Err(e) => {
            eprintln!("Error fetching schedule data: {:?}", e);
            let error_response = json!({
                "error": format!("Error fetching schedule data for station number {}", station_number),
                "details": format!("{}", e)
            });
            Json(error_response)
        }
    }
}
