#[must_use]
pub fn how_many_numbers_are_smaller_than_the_current_numbers_1(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .map(|&num| nums.iter().filter(|&n| *n < num).count())
        .collect()
}

#[must_use]
pub fn how_many_numbers_are_smaller_than_the_current_numbers_2(nums: &[i32]) -> Vec<usize> {
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort_unstable();

    nums.iter()
        .map(|num| sorted_nums.iter().position(|n| n == num).unwrap_or(0))
        .collect()
}
