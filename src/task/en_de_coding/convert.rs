#[must_use]
pub fn convert_binary_number_in_a_linked_list_to_integer(head: &[i32]) -> i32 {
    head.iter().fold(0, |num, &bit| (num << 1) + bit)
}
