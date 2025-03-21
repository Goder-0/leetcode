#[must_use]
pub fn decode_xored_array(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut ans = vec![0; encoded.len() + 1];
    ans[0] = first;
    for i in 0..encoded.len() {
        ans[i + 1] = ans[i] ^ encoded[i];
    }
    ans
}
