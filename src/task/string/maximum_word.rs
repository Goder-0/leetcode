#[must_use]
pub fn maximum_number_of_words_found_in_sentences(sentenses: &Vec<String>) -> i32 {
    let mut ans = 0;
    for sentense in sentenses {
        let v: Vec<&str> = sentense.split(' ').collect();
        ans = ans.max(v.len().try_into().unwrap_or(0));
    }
    ans
}
