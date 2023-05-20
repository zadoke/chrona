pub fn to_title_case(input_string: &str) -> String {
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