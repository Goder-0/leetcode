#[must_use]
pub fn jewels_and_stones(jewels: &str, stones: &str) -> i32 {
    stones
        .chars()
        .filter(|&c| jewels.contains(c))
        .count()
        .try_into()
        .unwrap_or(0)
}
