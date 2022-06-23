//! <https://www.codewars.com/kata/583ade15666df5a64e000058/train/rust>

pub fn evens_and_odds(n: u64) -> String {
    if n % 2 == 0 {
        format!("{:b}", n)
    } else {
        format!("{:x}", n)
    }
}
