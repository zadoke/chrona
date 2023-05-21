use axum::{extract::Path, response::IntoResponse, Json};
use serde_json::json;
use reqwest::Client;
use std::sync::Arc;

use crate::services::fetch_stations;

pub async fn handler(client: Arc<Client>,Path(query): Path<String>) -> impl IntoResponse {
    let query = query.split(' ').next().unwrap_or("").to_string(); // Remove extra words, the API will not respond to more than
    match fetch_stations(&client, &query).await {
        Ok(station_data) => Json(station_data),
        Err(e) => {
            eprintln!("Error fetching station data: {:?}", e);
            let error_response = json!({
                "error": format!("Error fetching stations"),
                "details": format!("{:?}", e)
            });
            Json(error_response)
        }
    }
}
