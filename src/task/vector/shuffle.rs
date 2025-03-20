#[must_use]
pub fn shuffle_the_array(nums: &[i32], n: usize) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::with_capacity(2 * n);
    for i in 0..n {
        ans.push(nums[i]);
        ans.push(nums[i + n]);
    }
    ans
}
