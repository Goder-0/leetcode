#[must_use]
pub fn build_array_from_permutation(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .filter_map(|&i| i.try_into().ok())
        .filter_map(|index: usize| nums.get(index))
        .filter_map(|&num| num.try_into().ok())
        .collect()
}
