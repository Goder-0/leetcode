#[must_use]
pub fn richest_customer_wealth(accounts: &[Vec<i32>]) -> Option<i32> {
    accounts.iter().map(|account| account.iter().sum()).max()
}
