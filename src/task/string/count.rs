#[must_use]
pub fn count_items_matching_a_rule(
    items: &Vec<Vec<String>>,
    rule_key: &str,
    rule_value: &str,
) -> i32 {
    let mut ans = 0;
    for item in items {
        match rule_key {
            "type" => ans += i32::from(*rule_value == item[0]),
            "color" => ans += i32::from(*rule_value == item[1]),
            "name" => ans += i32::from(*rule_value == item[2]),
            _ => {}
        }
    }

    ans
}
