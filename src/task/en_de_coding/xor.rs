#[must_use]
pub fn decode_xored_array(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = encoded
        .iter()
        .scan(first, |encode, &num| {
            *encode ^= num;
            Some(*encode)
        })
        .collect();
    ans.insert(0, first);
    ans
}
