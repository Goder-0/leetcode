#[must_use]
pub fn shuffle_string(s: &str, indices: &[i32]) -> String {
    let mut sc: Vec<(usize, char)> = s.chars().enumerate().collect();
    sc.sort_by_key(|(i, _)| indices[*i]);
    sc.iter().map(|(_, c)| c.to_string()).collect::<String>()
}
