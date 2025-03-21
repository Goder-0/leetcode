#[must_use]
pub fn create_target_array_in_the_given_order(nums: &[i32], index: &[i32]) -> Vec<i32> {
    let mut ans = Vec::new();
    for i in 0..nums.len() {
        ans.insert(index[i].try_into().unwrap_or(0), nums[i]);
    }

    ans
}
