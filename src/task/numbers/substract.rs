#[must_use]
pub fn subtract_the_product_and_sum_of_digits_of_an_integer(num: i32) -> u32 {
    let (sum, product) = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold((0, 1), |(s, p), n| (s + n, p * n));

    product - sum
}
