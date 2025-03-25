#[must_use]
pub fn running_sum_of_1d_array(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |s, &num| {
            *s += num;
            Some(*s)
        })
        .collect()
}
