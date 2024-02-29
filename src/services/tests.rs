use crate::models::train::Train;
use crate::models::train::Stops;

use serde_json::json;

#[test]
fn test_serialize_train_data() {
    let train_data = json!({
        "response": {
            "DataHoraDestino": "20-05-2023 12:30:00",
            "DataHoraOrigem": "20-05-2023 09:32:00",
            "Destino": "LISBOA-APOLÓNIA",
            "DuracaoViagem": "02:58",
            "NodesPassagemComboio": [
                {
                    "ComboioPassou": true,
                    "HoraProgramada": "09:32",
                    "NodeID": 9402006,
                    "NomeEstacao": "PORTO-CAMPANHÃ",
                    "Observacoes": ""
                },
                {
                    "ComboioPassou": true,
                    "HoraProgramada": "09:36",
                    "NodeID": 9439164,
                    "NomeEstacao": "GAIA-DEVESAS",
                    "Observacoes": ""
                },
                {
                    "ComboioPassou": true,
                    "HoraProgramada": "10:18",
                    "NodeID": 9438000,
                    "NomeEstacao": "AVEIRO",
                    "Observacoes": ""
                },
                {
                    "ComboioPassou": true,
                    "HoraProgramada": "10:44",
                    "NodeID": 9436004,
                    "NomeEstacao": "COIMBRA-B",
                    "Observacoes": ""
                },
                {
                    "ComboioPassou": true,
                    "HoraProgramada": "12:22",
                    "NodeID": 9431039,
                    "NomeEstacao": "LISBOA-ORIENTE",
                    "Observacoes": ""
                },
                {
                    "ComboioPassou": true,
                    "HoraProgramada": "12:30",
                    "NodeID": 9430007,
                    "NomeEstacao": "LISBOA-APOLÓNIA",
                    "Observacoes": ""
                }
            ],
            "Operador": "CP LONGO CURSO",
            "Origem": "PORTO-CAMPANHÃ",
            "SituacaoComboio": "Realizado",
            "TipoServico": "ALFA"
        }
    });

    let id = 550;

    let expected = Train {
        id,
        arrival_time: String::from("20-05-2023 12:30:00"),
        departure_time: String::from("20-05-2023 09:32:00"),
        destination: String::from("Lisboa Apolónia"),
        duration: String::from("02:58"),
        stops: vec![
            Stops {
                train_passed: true,
                scheduled_time: String::from("09:32"),
                id: 9402006,
                station_name: String::from("Porto Campanhã"),
                delay_info: String::from("Sem observações")
            },
            Stops {
                train_passed: true,
                scheduled_time: String::from("09:36"),
                id: 9439164,
                station_name: String::from("Gaia Devesas"),
                delay_info: String::from("Sem observações")
            },
            Stops {
                train_passed: true,
                scheduled_time: String::from("10:18"),
                id: 9438000,
                station_name: String::from("Aveiro"),
                delay_info: String::from("Sem observações")
            },
            Stops {
                train_passed: true,
                scheduled_time: String::from("10:44"),
                id: 9436004,
                station_name: String::from("Coimbra B"),
                delay_info: String::from("Sem observações")
            },
            Stops {
                train_passed: true,
                scheduled_time: String::from("12:22"),
                id: 9431039,
                station_name: String::from("Lisboa Oriente"),
                delay_info: String::from("Sem observações")
            },
            Stops {
                train_passed: true,
                scheduled_time: String::from("12:30"),
                id: 9430007,
                station_name: String::from("Lisboa Apolónia"),
                delay_info: String::from("Sem observações")
            }
        ],
        operator: String::from("CP LONGO CURSO"),
        origin: String::from("Porto Campanhã"),
        status: String::from("Realizado"),
        service_type: String::from("Alfa Pendular")
    };

    let result = Train::deserialize_train(&train_data, &id).unwrap();
    assert_eq!(result, expected);
}
