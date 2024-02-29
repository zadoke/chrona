use crate::utils::missing_field_error;
use crate::utils::to_title_case;
use crate::utils::map_service_type;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Train {
    pub id: i32,
    pub arrival_time: String,
    pub departure_time: String,
    pub destination: String,
    pub duration: String,
    pub stops: Vec<Stops>,
    pub operator: String,
    pub origin: String,
    pub status: String,
    pub service_type: String,
}

impl Train {
    pub fn deserialize_train(train_data: &Value, train_number: &i32) -> Result<Train, Box<dyn Error>> {
        let mut stops = Vec::new();
        if let Some(stops_data) = train_data["response"]["NodesPassagemComboio"].as_array() {
            for stop in stops_data.iter() {
                stops.push(Stops::deserialize_stops(stop)?);
            }
        } else {
            return Err(missing_field_error("NodesPassagemComboio").into());
        }

        Ok(Train {
            id : *train_number,
            arrival_time: train_data["response"]["DataHoraDestino"]
                .as_str()
                .ok_or_else(|| missing_field_error("DataHoraDestino"))?
                .to_string(),
            departure_time: train_data["response"]["DataHoraOrigem"]
                .as_str()
                .ok_or_else(|| missing_field_error("DataHoraOrigem"))?
                .to_string(),
            destination: to_title_case(
                train_data["response"]["Destino"].as_str().ok_or_else(|| missing_field_error("Destino"))?,
            ),
            duration: train_data["response"]["DuracaoViagem"]
                .as_str()
                .ok_or_else(|| missing_field_error("DuracaoViagem"))?
                .to_string(),
            stops,
            operator: train_data["response"]["Operador"]
                .as_str()
                .ok_or_else(|| missing_field_error("Operador"))?
                .to_string(),
            origin: to_title_case(
                train_data["response"]["Origem"].as_str().ok_or_else(|| missing_field_error("Origem"))?,
            ),
            status: match train_data["response"]["SituacaoComboio"].as_str() {
                Some(s) => if s.is_empty() { "Em circulação" } else { s }.to_string(),
                None => "Em circulação".to_string(),
            },
            service_type: map_service_type(train_data["response"]["TipoServico"]
                .as_str()
                .ok_or_else(|| missing_field_error("TipoServico"))?
            )?
        })
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stops {
    pub id: i32,
    pub train_passed: bool,
    pub scheduled_time: String,
    pub station_name: String,
    pub delay_info: String,
}

fn process_delay_info(s: &str) -> String {
    if s.contains("Hora Prevista") {
        s.split(':')
            .skip(1)
            .take(2)
            .map(|time| time.trim().to_string())
            .collect::<Vec<String>>()
            .join(":")
    } else if s.is_empty() {
        "Sem observações".to_string()
    } else {
        s.to_string()
    }
}

impl Stops {
    pub fn deserialize_stops(stops: &Value) -> Result<Stops, Box<dyn Error>> {
        Ok(Stops {
            train_passed: stops["ComboioPassou"]
                .as_bool()
                .ok_or("ComboioPassou is not a bool")?,
            scheduled_time: stops["HoraProgramada"]
                .as_str()
                .ok_or("HoraProgramada is not a string")?
                .to_string(),
            id: stops["NodeID"].as_i64().ok_or("NodeID is not an i64")? as i32,
            station_name: to_title_case(
                stops["NomeEstacao"]
                    .as_str()
                    .ok_or("NomeEstacao is not a string")?,
            ),
            delay_info: stops["Observacoes"]
            .as_str()
            .map(process_delay_info)
            .unwrap_or_else(|| "Sem observações".to_string()),
        })
    }
}