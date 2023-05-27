
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

use crate::models::station::Station;
use crate::utils::to_title_case;

fn transform_stations(station_data: &Value) -> Result<Vec<Station>, Box<dyn Error>>{
    let response = station_data["response"]
        .as_array()
        .ok_or("Invalid response")?;
    let stations: Vec<Station> = response
        .iter()
        .take(25)
        .map(|item| Station {
            name: to_title_case(item["Nome"].as_str().unwrap_or("")),
            id: item["NodeID"].as_i64().unwrap_or(0) as i32,
        })
        .collect();
    Ok(stations)
}


pub async fn fetch_stations(client: &Client, query: &String) -> Result<Value, Box<dyn Error>> {
    // Fetch train data from API using train number and current date
    let url = format!("https://servicos.infraestruturasdeportugal.pt/negocios-e-servicos/estacao-nome/{}", query);
    let response = client.get(url).send().await?.json::<Value>().await?;

    transform_stations(&response).and_then(|station_data| serde_json::to_value(station_data).map_err(|e| e.into()))
}

