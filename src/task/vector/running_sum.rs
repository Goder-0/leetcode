#[must_use]
pub fn running_sum_of_1d_array(nums: &[i32]) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![nums[0]];
    for i in 1..nums.len() {
        ans.push(ans[i - 1] + nums[i]);
    }
    ans
}
