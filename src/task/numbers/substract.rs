#[must_use]
pub fn subtract_the_product_and_sum_of_digits_of_an_integer(num: i32) -> u32 {
    let numbers: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect();

    let mut sum = 0;
    let mut product = 1;

    for i in numbers {
        sum += i;
        product *= i;
    }

    product - sum
}
