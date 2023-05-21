use axum::{extract::Path, response::IntoResponse, Json};
use serde_json::json;
use reqwest::Client;
use std::sync::Arc;

use crate::services::fetch_train_data;

pub async fn handler(client: Arc<Client>,Path(train_number): Path<i32>) -> impl IntoResponse {
    match fetch_train_data(&client, &train_number).await {
        Ok(train_data) => Json(train_data),
        Err(e) => {
            eprintln!("Error fetching train data: {:?}", e);
            let error_response = json!({
                "error": format!("Error fetching train data for train number {}", train_number),
                "details": format!("{:?}", e)
            });
            Json(error_response)
        }
    }
}
