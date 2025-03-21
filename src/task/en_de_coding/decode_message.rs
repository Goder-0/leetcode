use std::collections::HashMap;

#[must_use]
pub fn decode_the_message(key: &str, message: &str) -> String {
    let abc = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut dict: HashMap<char, char> = HashMap::new();
    let mut idx = 0;
    dict.insert(' ', ' ');

    for c in key.chars() {
        dict.entry(c).or_insert_with(|| {
            let value = abc.chars().nth(idx).unwrap_or('x');
            idx += 1;
            value
        });
    }

    message
        .chars()
        .map(|c| dict.get(&c).unwrap_or(&'x').to_string())
        .collect::<String>()
}
