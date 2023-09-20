use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Europe::Lisbon;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

use crate::models::schedule::Schedule;

pub async fn fetch_schedule_data(
    client: &Client,
    station_number: &i32,
    schedule_type: usize,
) -> Result<Value, Box<dyn Error>> {
    let url = generate_url(station_number);
    let response = client.get(&url).send().await?.json::<Value>().await?;
    handle_response(response, schedule_type)
}

fn generate_url(station_number: &i32) -> String {
    // Get current date in Lisbon timezone
    let current_date = Utc::now()
        .with_timezone(&Lisbon)
        .format("%Y-%m-%d %H:%M")
        .to_string();
    let future_date = (Utc::now() + Duration::hours(12))
        .with_timezone(&Lisbon)
        .format("%Y-%m-%d %H:%M")
        .to_string(); //Add 12 hours to the current time

    format!("https://servicos.infraestruturasdeportugal.pt/negocios-e-servicos/partidas-chegadas/{}/{}/{}/INTERNACIONAL,%20ALFA,%20IC,%20IR,%20REGIONAL,%20URB|SUBUR,%20ESPECIAL", station_number, current_date, future_date)
}

fn handle_response(response: Value, schedule_type: usize) -> Result<Value, Box<dyn Error>> {
    if let Some(response) = response["response"].as_array() {
        if response.is_empty() {
            return Ok(json!({
                "error": "No Trains / Station not found",
                "details": "Não há partidas/chegadas na estação selecionada."
            }));
        }
    }

    Schedule::from_value(&response, schedule_type)
        .and_then(|schedule_data| serde_json::to_value(schedule_data).map_err(|e| e.into()))
}
