pub mod task;

#[cfg(test)]
mod tests {

    use std::vec;

    use task::*;

    use super::*;
    use crate::task::en_de_coding::ListNode;

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

    #[test]
    fn pair() {
        let v = vec![1, 2, 3, 1, 1, 3];
        assert_eq!(numbers::pair::number_of_good_pairs(&v), 4);
    }

    #[test]
    fn candy() {
        let v = vec![2, 3, 5, 1, 3];
        assert_eq!(
            numbers::candy::kids_with_the_greatest_number_of_cnadies(&v, 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn substract() {
        assert_eq!(
            numbers::substract::subtract_the_product_and_sum_of_digits_of_an_integer(234),
            15
        );
        assert_eq!(
            numbers::substract::subtract_the_product_and_sum_of_digits_of_an_integer(4421),
            21
        );
    }

    #[test]
    fn small_count() {
        let v = [8, 1, 2, 2, 3];
        assert_eq!(
            numbers::small_count::how_many_numbers_are_smaller_than_the_current_numbers_1(&v),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            numbers::small_count::how_many_numbers_are_smaller_than_the_current_numbers_2(&v),
            vec![4, 0, 1, 1, 3]
        )
    }

    #[test]
    fn xor() {
        let v = vec![6, 2, 7, 3];
        assert_eq!(
            en_de_coding::xor::decode_xored_array(&v, 4),
            vec![4, 2, 0, 7, 4]
        )
    }

    #[test]
    fn decompress() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(
            en_de_coding::decompress::decompress_run_length_encoded_list(&v),
            vec![2, 4, 4, 4]
        );
    }

    #[test]
    fn shuffle_string() {
        let s = "codeleet";
        let v = vec![4, 5, 6, 7, 0, 2, 1, 3];
        assert_eq!(
            en_de_coding::shuffle_string::shuffle_string(s, &v),
            "leetcode".to_string()
        );
    }

    #[test]
    fn decode_message() {
        let s1 = "the quick brown fox jumps over the lazy dog";
        let s2 = "vkbs bs t suepuv";
        assert_eq!(
            en_de_coding::decode_message::decode_the_message(s1, s2),
            "this is a secret"
        );
    }

    #[test]
    fn convert() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(1))),
            })),
        }));

        assert_eq!(
            en_de_coding::convert::convert_binary_number_in_a_linked_list_to_integer(&list),
            5
        );
    }

    #[test]
    fn after_performing() {
        let v = vec!["--X", "X++", "X++"];
        assert_eq!(
            iterations::after_performing::final_value_of_variable_after_performing_operations(&v),
            1
        );
    }

    #[test]
    fn reduce() {
        assert_eq!(
            iterations::reduce::number_of_steps_to_redeuce_a_number_to_zero(14),
            6
        );
    }

    #[test]
    fn create_array() {
        assert_eq!(
            iterations::create_array::create_target_array_in_the_given_order(
                &[0, 1, 2, 3, 4],
                &[0, 1, 2, 2, 1]
            ),
            vec![0, 4, 1, 3, 2]
        );
    }
}
