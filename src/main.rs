use chrono::prelude::*;
use chrono_tz::Europe::Lisbon;
use reqwest;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrainData {
    arrival_time: String,
    departure_time: String,
    destination: String,
    duration: String,
    stops: Vec<TrainStop>,
    operator: String,
    origin: String,
    status: String,
    service_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrainStop {
    train_passed: bool,
    scheduled_time: String,
    node_id: i32,
    station_name: String,
    delay_info: String,
}

fn to_title_case(input_string: &str) -> String {
    input_string
        .split(|character| character == ' ' || character == '-' || character =='.')
        .map(|word| {
            let mut characters = word.chars();
            match characters.next() {
                None => String::new(),
                Some(first_character) => first_character.to_uppercase().collect::<String>() + characters.as_str().to_lowercase().as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

//This function transforms the train data from the API response into a more readable form
fn transform_train_data(train_data: &Value) -> Result<TrainData, Box<dyn Error>> {
    let stops = train_data["response"]["NodesPassagemComboio"]
        .as_array()
        .ok_or("NodesPassagemComboio is not an array")?
        .iter()
        .map(|stop| -> Result<TrainStop, Box<dyn Error>> {
            Ok(TrainStop {
                train_passed: stop["ComboioPassou"]
                    .as_bool()
                    .ok_or("ComboioPassou is not a bool")?,
                scheduled_time: stop["HoraProgramada"]
                    .as_str()
                    .ok_or("HoraProgramada is not a string")?
                    .to_string(),
                node_id: stop["NodeID"]
                    .as_i64()
                    .ok_or("NodeID is not an i64")? as i32,
                station_name: to_title_case(stop["NomeEstacao"]
                    .as_str()
                    .ok_or("NomeEstacao is not a string")?
                ),
                delay_info: stop["Observacoes"]
                    .as_str()
                    .ok_or("Observacoes is not a string")?
                    .to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TrainData {
        arrival_time: train_data["response"]["DataHoraDestino"]
        .as_str()
        .ok_or("DataHoraDestino is not a string")?
        .to_string(),
       departure_time: train_data["response"]["DataHoraOrigem"]
        .as_str()
        .ok_or("DataHoraOrigem is not a string")?
        .to_string(),
        destination: to_title_case(train_data["response"]["Destino"]
            .as_str()
            .ok_or("Destino is not a string")?
        ),
        duration: train_data["response"]["DuracaoViagem"]
            .as_str()
            .ok_or("DuracaoViagem is not a string")?
            .to_string(),
        stops,
        operator: train_data["response"]["Operador"]
            .as_str()
            .ok_or("Operador is not a string")?
            .to_string(),
        origin: to_title_case(train_data["response"]["Origem"]
            .as_str()
            .ok_or("Origem is not a string")?
        ),
        status: train_data["response"]["SituacaoComboio"]
            .as_str()
            .ok_or("SituacaoComboio is not a string")?
            .to_string(),
        service_type: train_data["response"]["TipoServico"]
            .as_str()
            .ok_or("TipoServico is not a string")?
            .to_string(),
    })
}

// This function fetches train data from the API using the train number and current date
async fn fetch_train_data(train_number: &i32) -> Result<Value, Box<dyn std::error::Error>> {
    // Get current date in Lisbon timezone
    let current_date = Utc::now().with_timezone(&Lisbon).format("%Y-%m-%d").to_string();

    // Fetch train data from API using train number and current date
    let url = format!("https://servicos.infraestruturasdeportugal.pt/negocios-e-servicos/horarios-ncombio/{}/{}", train_number, current_date);
    let response = reqwest::get(url).await?.json::<Value>().await?;

    // Check if the train exists
    if response["response"]["DataHoraDestino"].is_null() {
        Ok(json!({ "trainNotFound": true, "message": "O comboio não foi encontrado." })) // Needs reworking
    } else {
        transform_train_data(&response).and_then(|train_data| serde_json::to_value(train_data).map_err(|e| e.into()))
    }
}


#[tokio::main]
async fn main() {
    match fetch_train_data(&18012).await { //temporary number for testing
        Ok(train_data) => println!("{}", train_data),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}