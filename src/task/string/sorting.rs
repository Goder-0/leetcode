#[must_use]
pub fn sorting_the_sentence(s: &str) -> String {
    let mut words: Vec<(&str, u32)> = s
        .split_whitespace()
        .filter_map(|word| {
            let (text, last) = word.split_at(word.len() - 1);
            match last.parse() {
                Ok(id) => Some((text, id)),
                Err(_) => None,
            }
        })
        .collect();

    words.sort_unstable_by_key(|&(_, idx)| idx);

    words
        .into_iter()
        .map(|(w, _)| w.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
