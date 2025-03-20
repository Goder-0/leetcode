#[must_use]
pub fn sorting_the_sentence(s: &str) -> String {
    let mut words: Vec<(String, u32)> = s
        .split_whitespace()
        .map(|word| {
            let c = word.chars().last();
            let order = c.unwrap_or('0').to_digit(10).unwrap_or(0);
            (word[..word.len() - 1].to_string(), order)
        })
        .collect();

    words.sort_by_key(|&(_, order)| order);

    words
        .into_iter()
        .map(|(w, _)| w)
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}
