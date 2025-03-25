#[must_use]
pub fn build_array_from_permutation(nums: &[usize]) -> Vec<usize> {
    nums.iter()
        .filter_map(|&index| nums.get(index))
        .copied()
        .collect()
}
