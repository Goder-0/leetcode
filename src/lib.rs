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

    #[test]
    fn richest() {
        let v = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(vector::richest::richest_customer_wealth(&v), 17);
    }

    #[test]
    fn shuffle() {
        let v = vec![2, 5, 1, 3, 4, 7];
        assert_eq!(
            vector::shuffle::shuffle_the_array(&v, 3),
            vec![2, 3, 5, 4, 1, 7]
        );
    }

    #[test]
    fn ip_address() {
        let s = "128.0.0.1".to_string();
        assert_eq!(
            string::ip_address::defanging_an_ip_address(&s),
            "128[.]0[.]0[.]1".to_string()
        );
    }

    #[test]
    fn jewel_stone() {
        let s1 = "aAb".to_string();
        let s2 = "aAaAzzzzBBBBbb".to_string();
        assert_eq!(string::jewel_stone::jewels_and_stones(&s1, &s2), 6);
    }

    #[test]
    fn maximum_word() {
        let v = vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ];
        assert_eq!(
            string::maximum_word::maximum_number_of_words_found_in_sentences(&v),
            6
        );
    }

    #[test]
    fn sorting() {
        let s = "is2 sentence4 This1 a3".to_string();
        assert_eq!(
            string::sorting::sorting_the_sentence(&s),
            "This is a sentence".to_string()
        );
    }

    #[test]
    fn count() {
        let v = vec![
            vec!["phone", "blue", "pixel"],
            vec!["computer", "silver", "phone"],
            vec!["phone", "gold", "iphone"],
        ];

        let v: Vec<Vec<String>> = v
            .into_iter()
            .map(|inner| {
                inner
                    .into_iter()
                    .map(std::string::ToString::to_string)
                    .collect()
            })
            .collect();

        assert_eq!(
            string::count::count_items_matching_a_rule(&v, "type", "phone"),
            2
        );
    }

    #[test]
    fn minimum_sum() {
        assert_eq!(
            numbers::minimum_sum::minimum_sum_of_four_digit_number_after_splitting_digits(2932),
            52
        );
    }
}
