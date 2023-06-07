
use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Europe::Lisbon;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

use crate::models::schedule::Schedule;
use crate::models::schedule::Train;
use crate::utils::to_title_case;

fn transform_schedule_data(schedule_data: &Value, schedule_type: usize) -> Result<Schedule, Box<dyn Error>> {
    if let Some(response) = schedule_data.get("response") {
        if let Some(data) = response.get(schedule_type) {
            return Ok(map_schedule(data)?);
        }
    }
    Err("Error mapping station data".into())
}

fn map_schedule(schedule_data: &Value) -> Result<Schedule, Box<dyn Error>> {
    let station_id = schedule_data["NodeID"].as_u64().ok_or("Missing NodeID")? as u32;
    let station_name = to_title_case(schedule_data["NomeEstacao"].as_str().ok_or("Missing NomeEstacao")?);
    
    let mut trains = Vec::new();
    
    match schedule_data.get("NodesComboioTabelsPartidasChegadas") {
        Some(train_data) => {
            for train in train_data.as_array().ok_or("NodesComboioTabelsPartidasChegadas is not an array")? {
                let train_passed = train["ComboioPassou"].as_bool().ok_or("Missing ComboioPassou")?;
                let time = train["DataHoraPartidaChegada"].as_str().ok_or("Missing DataHoraPartidaChegada")?.to_string();
                let date = train["DataRealizacao"].as_str().ok_or("Missing DataRealizacao")?.to_string();
                let destination_station_id = train["EstacaoDestino"].as_u64().ok_or("Missing EstacaoDestino")? as u32;
                let origin_station_id = train["EstacaoOrigem"].as_u64().ok_or("Missing EstacaoOrigem")? as u32;
                let train_number_1 = train["NComboio1"].as_u64().ok_or("Missing NComboio1")? as u32;
                let train_number_2 = train["NComboio2"].as_u64().ok_or("Missing NComboio2")? as u32;
                let destination_station_name = to_title_case(train["NomeEstacaoDestino"].as_str().ok_or("Missing NomeEstacaoDestino")?);
                let origin_station_name = to_title_case(train["NomeEstacaoOrigem"].as_str().ok_or("Missing NomeEstacaoOrigem")?);
                let operator = train["Operador"].as_str().ok_or("Missing Operador")?.to_string();
                let service_type = train["TipoServico"].as_str().ok_or("Missing TipoServico")?.to_string();
                let info = train["Observacoes"].as_str().map(|s| if s.is_empty() { "Sem atraso" } else { s })
                .unwrap_or("Sem atraso").to_string();
                trains.push(Train {
                    train_passed,
                    time,
                    date,
                    destination_station_id,
                    origin_station_id,
                    train_number: if train_number_1 != 0 {train_number_1} else {train_number_2},
                    destination_station_name,
                    origin_station_name,
                    operator,
                    service_type,
                    info,
                    carriages: if train_number_1 != train_number_2 {8} else {4},
                });
            }
        }
        None => {}
    }

    Ok(Schedule { station_id, trains, station_name })
}

pub async fn fetch_schedule_data(client: &Client, station_number: &i32, schedule_type: usize) -> Result<Value, Box<dyn Error>> {
    // Get current date in Lisbon timezone
    let current_date = Utc::now().with_timezone(&Lisbon).format("%Y-%m-%d %H:%M").to_string();
    let future_date = (Utc::now() + Duration::hours(12)).with_timezone(&Lisbon).format("%Y-%m-%d %H:%M").to_string(); //Add 12 hours to the current time

    let url = format!("https://servicos.infraestruturasdeportugal.pt/negocios-e-servicos/partidas-chegadas/{}/{}/{}/INTERNACIONAL,%20ALFA,%20IC,%20IR,%20REGIONAL,%20URB|SUBUR,%20ESPECIAL", station_number, current_date, future_date);
    let response = client.get(url).send().await?.json::<Value>().await?;


    if let Some(response) = response["response"].as_array() {
        if response.is_empty() {
            return Ok(json!({
                "error": "No Trains / Station not found",
                "details": "Não há partidas/chegadas na estação selecionada."
            }));
        }
    }
    
    transform_schedule_data(&response, schedule_type).and_then(|schedule_data| serde_json::to_value(schedule_data).map_err(|e| e.into()))
}




