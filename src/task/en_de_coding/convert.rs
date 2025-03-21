use super::ListNode;

#[must_use]
pub fn convert_binary_number_in_a_linked_list_to_integer(head: &Option<Box<ListNode>>) -> u32 {
    let mut s = String::new();
    let mut current = head.as_deref();
    while let Some(node) = current {
        s.push_str(&node.val.to_string());
        current = node.next.as_deref();
    }

    u32::from_str_radix(&s, 2).unwrap_or(0)
}
