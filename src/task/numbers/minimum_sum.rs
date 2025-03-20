#[must_use]
pub fn minimum_sum_of_four_digit_number_after_splitting_digits(num: i32) -> u32 {
    let mut ans = u32::MAX;
    let nums: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect();

    for i in 0..4 {
        let mut indices: Vec<usize> = (0..4).collect();
        indices.retain(|&x| x != i); // i 제외

        for &j in &indices {
            let mut indices_j = indices.clone();
            indices_j.retain(|&x| x != j); // j 제외

            for &k in &indices_j {
                let mut indices_k = indices_j.clone();
                indices_k.retain(|&x| x != k); // k 제외

                for &l in &indices_k {
                    ans = ans.min(sum_of_numbers(((nums[i], nums[j]), (nums[k], nums[l]))));
                }
            }
        }
    }

    ans
}

fn sum_of_numbers(numbers: ((u32, u32), (u32, u32))) -> u32 {
    let ((a, b), (c, d)) = numbers;
    (a + c) * 10 + b + d
}
