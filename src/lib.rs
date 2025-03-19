pub mod task;

#[cfg(test)]
mod tests {
    use task::*;

    use super::*;
    #[test]
    fn add() {
        let result = add::sum(2, 2);
        assert_eq!(result, 4);
    }

    fn concatenation() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];
        vector::concatenation::concatenation_of_array(v, 5);
    }
}
