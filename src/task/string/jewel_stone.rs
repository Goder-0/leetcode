#[must_use]
pub fn jewels_and_stones(jewels: &str, stones: &str) -> Option<i32> {
    stones
        .chars()
        .filter(|&c| jewels.contains(c))
        .count()
        .try_into()
        .ok()
}
