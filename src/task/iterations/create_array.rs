#[must_use]
pub fn create_target_array_in_the_given_order(nums: &[i32], index: &[usize]) -> Vec<i32> {
    let mut ans = Vec::new();
    nums.iter().zip(index.iter()).for_each(|(&num, &idx)| {
        ans.insert(idx, num);
    });

    ans
}
