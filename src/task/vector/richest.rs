#[must_use]
pub fn richest_customer_wealth(accounts: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for account in accounts {
        let sum = account.iter().sum();
        ans = ans.max(sum);
    }
    ans
}
