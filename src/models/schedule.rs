use crate::utils::missing_field_error;
use crate::utils::to_title_case;
use crate::utils::map_service_type;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    station_id: u32,
    trains: Vec<Train>,
    station_name: String,
}

impl Schedule {
    pub fn from_value(
        schedule_data: &Value,
        schedule_type: usize,
    ) -> Result<Schedule, Box<dyn Error>> {
        if let Some(response) = schedule_data.get("response") {
            if let Some(data) = response.get(schedule_type) {
                let station_id = data["NodeID"]
                    .as_u64()
                    .ok_or_else(|| missing_field_error("NodeID"))?
                    as u32;
                let station_name = to_title_case(
                    data["NomeEstacao"]
                        .as_str()
                        .ok_or_else(|| missing_field_error("NomeEstacao"))?,
                );

                let trains = data.get("NodesComboioTabelsPartidasChegadas").map_or_else(
                    Vec::new,
                    |train_data| {
                        train_data
                            .as_array()
                            .unwrap_or(&Vec::new())
                            .iter()
                            .filter_map(|train| Train::from_value(train).ok())
                            .collect()
                    },
                );

                return Ok(Schedule {
                    station_id,
                    trains,
                    station_name,
                });
            }
        }
        Err("Error mapping station data".into())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Train {
    train_passed: bool,
    time: String,
    date: String,
    destination_station_id: u32,
    origin_station_id: u32,
    train_number: u32,
    destination_station_name: String,
    origin_station_name: String,
    operator: String,
    service_type: String,
    info: String,
    carriages: i32,
}

impl Train {
    fn from_value(train: &Value) -> Result<Train, Box<dyn Error>> {
        fn map_info(info: Option<&str>) -> Result<String, Box<dyn Error>> {
            match info {
                Some(s) if !s.is_empty() => Ok(s.to_string()),
                _ => Ok("Sem atraso".to_string()),
            }
        }

        let train_passed = train["ComboioPassou"]
            .as_bool()
            .ok_or_else(|| missing_field_error("ComboioPassou"))?;
        let time = train["DataHoraPartidaChegada"]
            .as_str()
            .ok_or_else(|| missing_field_error("DataHoraPartidaChegada"))?
            .to_string();
        let date = train["DataRealizacao"]
            .as_str()
            .ok_or_else(|| missing_field_error("DataRealizacao"))?
            .to_string();
        let destination_station_id = train["EstacaoDestino"]
            .as_u64()
            .ok_or_else(|| missing_field_error("EstacaoDestino"))?
            as u32;
        let origin_station_id = train["EstacaoOrigem"]
            .as_u64()
            .ok_or_else(|| missing_field_error("EstacaoOrigem"))?
            as u32;
        let train_number_1 = train["NComboio1"]
            .as_u64()
            .ok_or_else(|| missing_field_error("NComboio1"))? as u32;
        let train_number_2 = train["NComboio2"]
            .as_u64()
            .ok_or_else(|| missing_field_error("NComboio2"))? as u32;
        let destination_station_name = to_title_case(
            train["NomeEstacaoDestino"]
                .as_str()
                .ok_or_else(|| missing_field_error("NomeEstacaoDestino"))?,
        );
        let origin_station_name = to_title_case(
            train["NomeEstacaoOrigem"]
                .as_str()
                .ok_or_else(|| missing_field_error("NomeEstacaoOrigem"))?,
        );
        let operator = train["Operador"]
            .as_str()
            .ok_or_else(|| missing_field_error("Operador"))?
            .to_string();
        let service_type = map_service_type(
            train["TipoServico"]
                .as_str()
                .ok_or_else(|| missing_field_error("TipoServico"))?,
        )?;
        let info = map_info(train["Observacoes"].as_str())?;

        Ok(Train {
            train_passed,
            time,
            date,
            destination_station_id,
            origin_station_id,
            train_number: if train_number_1 != 0 {
                train_number_1
            } else {
                train_number_2
            },
            destination_station_name,
            origin_station_name,
            operator,
            service_type,
            info,
            carriages: if train_number_1 != train_number_2 {
                8
            } else {
                4
            },
        })
    }
}
