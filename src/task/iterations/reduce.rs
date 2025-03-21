#[must_use]
pub fn number_of_steps_to_redeuce_a_number_to_zero(num: i32) -> i32 {
    let mut n = num;
    let mut ans = 0;

    while n != 0 {
        match n % 2 {
            0 => n /= 2,
            1 => n -= 1,
            _ => unreachable!(),
        }
        ans += 1;
    }

    ans
}
