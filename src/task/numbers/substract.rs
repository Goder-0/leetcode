#[must_use]
pub fn subtract_the_product_and_sum_of_digits_of_an_integer(num: i32) -> u32 {
    let numbers = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .fold((0, 1), |state: (u32, u32), number| {
            (state.0 + number, state.1 * number)
        });
    numbers.1 - numbers.0
}
