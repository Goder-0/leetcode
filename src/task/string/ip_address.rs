#[must_use]
pub fn defanging_an_ip_address(address: &str) -> String {
    address.replace('.', "[.]")
}
