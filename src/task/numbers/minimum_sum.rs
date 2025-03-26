#[must_use]
pub fn minimum_sum_of_four_digit_number_after_splitting_digits(num: i32) -> u32 {
    let n = num.to_string();
    let mut nums: Vec<u32> = n.chars().filter_map(|c| c.to_digit(10)).collect();
    nums.sort_unstable();
    nums.iter().enumerate().fold(0, |mut ans, (idx, num)| {
        ans += if idx < 2 { num * 10 } else { *num };
        ans
    })
}
