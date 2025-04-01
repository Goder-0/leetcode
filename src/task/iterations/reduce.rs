#[must_use]
pub fn number_of_steps_to_reduce_a_number_to_zero(num: u32) -> u32 {
    u32::BITS - num.leading_zeros() + num.count_ones() - u32::from(num != 0)
}
