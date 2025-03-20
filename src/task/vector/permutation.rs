#[must_use]
pub fn build_array_from_permutation(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .map(|&num| {
            let index = usize::try_from(num).unwrap_or(0);
            nums.get(index).copied().unwrap_or(0)
        })
        .collect()
}
