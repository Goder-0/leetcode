pub mod task;

#[cfg(test)]
mod tests {

    use task::*;

    use super::*;

    #[test]
    fn add() {
        assert_eq!(8, task::add::sum(3, 5));
        assert_eq!(200, task::add::sum(100, 100));
    }

    #[test]
    fn concatenation() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(
            vector::concatenation::concatenation_of_array(&v),
            vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn permutation() {
        let v = vec![0, 2, 1, 5, 3, 4];
        assert_eq!(
            vector::permutation::build_array_from_permutation(&v),
            [0, 1, 2, 4, 5, 3]
        );
    }

    #[test]
    fn running_sum() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(
            vector::running_sum::running_sum_of_1d_array(&v),
            vec![1, 3, 6, 10, 15]
        );
    }
}
