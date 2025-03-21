#[must_use]
pub fn kids_with_the_greatest_number_of_cnadies(candies: &Vec<i32>, extra_candy: i32) -> Vec<bool> {
    let mut ans: Vec<bool> = Vec::with_capacity(candies.len());
    let max_value = candies.iter().max();
    for candy in candies {
        if let Some(&max_candy) = max_value {
            ans.push(candy + extra_candy >= max_candy);
        }
    }
    ans
}
