use std::collections::HashMap;
#[must_use]
pub fn number_of_good_pairs(nums: &[i32]) -> Option<i32> {
    let mut pairs = HashMap::new();
    let mut sum = 0;
    for &num in nums {
        pairs
            .entry(num)
            .and_modify(|n| {
                sum += *n;
                *n += 1;
            })
            .or_insert(1);
    }
    Some(sum)
}
