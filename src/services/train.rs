use chrono::prelude::*;
use chrono_tz::Europe::Lisbon;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

use crate::models::train::Train;

fn generate_url(train_number: &i32) -> String {
    // Get current date in Lisbon timezone
    let current_date = Utc::now()
        .with_timezone(&Lisbon)
        .format("%Y-%m-%d")
        .to_string();
    format!("https://servicos.infraestruturasdeportugal.pt/negocios-e-servicos/horarios-ncombio/{}/{}", train_number, current_date)
}

pub async fn fetch_train_data(client: &Client, train_number: &i32) -> Result<Value, Box<dyn Error>> {
    let url = generate_url(train_number);
    let response = client.get(&url).send().await?.json::<Value>().await?;
    handle_response(response, train_number)
}

fn handle_response(response: Value, train_number: &i32) -> Result<Value, Box<dyn Error>> {
    if let Some(response) = response["response"].as_object() {
        if response.values().all(|v| v.is_null()) { // The API returns null on all fields when it can't find a train
            return Ok(json!({
                "error": "Train not found",
                "details": "O comboio não foi encontrado"
            }));
        }
    }

    // Convert the response to a Train object
    let train_result = Train::deserialize_train(&response, train_number);

    // If successful, convert the Train object to a JSON Value
    let json_result = train_result.and_then(|train_data| {
        serde_json::to_value(train_data).map_err(|e| e.into())
    });

    json_result
}
