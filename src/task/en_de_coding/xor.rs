#[must_use]
pub fn decode_xored_array(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut ans = Vec::with_capacity(encoded.len() + 1); // 최적화된 벡터 생성
    ans.push(first);

    encoded
        .iter()
        .scan(first, |state, &num| {
            *state ^= num;
            Some(*state)
        })
        .for_each(|n| ans.push(n)); // collect() 없이 직접 push

    ans
}
