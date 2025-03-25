#[must_use]
pub fn richest_customer_wealth(accounts: &[Vec<i32>]) -> i32 {
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .unwrap_or(0)
}
