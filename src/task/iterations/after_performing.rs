#[must_use]
pub fn final_value_of_variable_after_performing_operations(operations: &Vec<&str>) -> i32 {
    let mut x = 0;
    for operation in operations {
        operation.contains('+').then(|| x += 1);
        operation.contains('-').then(|| x -= 1);
    }

    x
}
