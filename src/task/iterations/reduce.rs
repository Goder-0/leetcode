#[must_use]
pub fn number_of_steps_to_reduce_a_number_to_zero(num: u32) -> u32 {
    31 + num.count_ones() - num.leading_zeros()
}
