//! <https://www.codewars.com/kata/583ade15666df5a64e000058/train/rust>

use digital::NumToString;

pub fn evens_and_odds(n: u64) -> String {
    if n % 2 == 0 {
        n.to_string_base2(false, false)
    } else {
        n.to_string_base16(false, false)
    }
}
