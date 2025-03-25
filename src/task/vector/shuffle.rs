#[must_use]
pub fn shuffle_the_array(nums: &[i32], n: usize) -> Vec<i32> {
    let (arr_x, arr_y) = nums.split_at(n);
    arr_x
        .iter()
        .zip(arr_y.iter())
        .flat_map(|(&x, &y): (&i32, &i32)| [x, y])
        .collect()
}
