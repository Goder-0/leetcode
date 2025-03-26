use std::collections::HashMap;
#[must_use]
pub fn number_of_good_pairs(nums: &[i32]) -> Option<i32> {
    let mut pairs = HashMap::new();
    for &num in nums {
        pairs.entry(num).and_modify(|n| *n += *n).or_insert(1);
    }
    pairs.values().max().copied()
}
