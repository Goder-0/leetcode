#[must_use]
pub fn number_of_good_pairs(nums: &[i32]) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] == nums[j] {
                ans += 1;
            }
        }
    }
    ans
}
