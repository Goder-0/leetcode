#[allow(clippy::missing_errors_doc)]
pub fn count_items_matching_a_rule(
    items: &[Vec<String>],
    rule_key: &str,
    rule_value: &str,
) -> Result<i32, String> {
    let index = match rule_key {
        "type" => Some(0),
        "color" => Some(1),
        "name" => Some(2),
        _ => None,
    };

    index
        .map(|i| {
            items
                .iter()
                .filter(|&item| item.get(i).is_some_and(|v| v == rule_value))
                .count()
        })
        .ok_or_else(|| "Error".to_string())?
        .try_into()
        .map_err(|_| "Error".to_string())
}
