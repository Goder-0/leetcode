#[must_use]
pub fn convert_binary_number_in_a_linked_list_to_integer(head: &[i32]) -> u32 {
    let num = u32::from_str_radix(
        &head
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>(),
        2,
    );

    num.unwrap_or_default()
}
