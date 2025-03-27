#[must_use]
pub fn decompress_run_length_encoded_list(nums: &[i32]) -> Vec<i32> {
    nums.chunks(2)
        .filter_map(|ch| ch.first().zip(ch.get(1)))
        .filter_map(|(&freq, &num)| (freq.try_into().ok()).map(|f| vec![num; f]))
        .flatten()
        .collect()
}
