#[must_use]
pub fn kids_with_the_greatest_number_of_cnadies(
    candies: &[i32],
    extra_candy: i32,
) -> Option<Vec<bool>> {
    let max_value = candies.iter().max();
    max_value.map(|max: &i32| {
        candies
            .iter()
            .map(|&candy: &i32| candy + extra_candy >= *max)
            .collect()
    })
}
