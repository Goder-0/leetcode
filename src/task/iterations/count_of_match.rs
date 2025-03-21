#[must_use]
pub fn count_of_matches_in_tournament(n: i32) -> i32 {
    let mut count = 0;
    let mut m = n;

    while m > 1 {
        count += m / 2;
        m = (m + 1) / 2;
    }

    count
}
