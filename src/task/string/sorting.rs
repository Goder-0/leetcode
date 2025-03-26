#[must_use]
pub fn sorting_the_sentence(s: &str) -> String {
    let mut words: Vec<(&str, char)> = s
        .split_whitespace()
        .filter_map(|word| {
            let (text, last) = word.split_at(word.len() - 1);
            last.chars().next().map(|c| (text, c))
        })
        .collect();

    words.sort_unstable_by_key(|&(_, c)| c.to_digit(10).unwrap_or(0));

    words
        .into_iter()
        .map(|(w, _)| w.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
