#[must_use]
pub fn decompress_run_length_encoded_list(nums: &[i32]) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let frequents: Vec<(i32, i32)> = nums.chunks(2).map(|ch| (ch[0], ch[1])).collect();

    for (freq, value) in frequents {
        for _ in 0..freq {
            ans.push(value);
        }
    }

    ans
}
