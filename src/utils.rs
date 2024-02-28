use std::error::Error;

pub fn map_service_type(service_type: &str) -> Result<String, Box<dyn Error>> {
    match service_type {
        "IC" => Ok("Intercidades".to_string()),
        "ALFA" => Ok("Alfa Pendular".to_string()),
        "REGIONAL" => Ok("Regional".to_string()),
        "URB|SUBUR" => Ok("Suburbano".to_string()),
        "IR" => Ok("InterRegional".to_string()),
        "MERCADORIAS" => Ok("Carga".to_string()),
        "SERVIÇO" => Ok("Serviço".to_string()),
        "ESPECIAL" => Ok("Especial".to_string()),
        _ => Ok("Desconhecido".to_string()),
    }
}

pub fn to_title_case(input_string: &str) -> String {
    input_string
        .split(|character| character == ' ' || character == '-' || character == '.')
        .map(|word| {
            let mut characters = word.chars();
            match characters.next() {
                None => String::new(),
                Some(first_character) => {
                    first_character.to_uppercase().collect::<String>()
                        + characters.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn missing_field_error(field_name: &str) -> Box<dyn Error> {
    format!("Failed to map data. {} is missing.", field_name).into()
}
