#[must_use]
pub fn how_many_numbers_are_smaller_than_the_current_numbers_1(nums: &[i32]) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] > nums[j] {
                ans[i] += 1;
            }
        }
    }
    ans
}

#[must_use]
pub fn how_many_numbers_are_smaller_than_the_current_numbers_2(nums: &[i32]) -> Vec<usize> {
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort_unstable();

    nums.iter()
        .map(|num| sorted_nums.iter().position(|n| n == num).unwrap_or(0))
        .collect()
}
