#[must_use]
pub fn maximum_number_of_words_found_in_sentences(sentenses: &[String]) -> Option<usize> {
    sentenses
        .iter()
        .map(|sentense| sentense.split(' ').count())
        .max()
}
