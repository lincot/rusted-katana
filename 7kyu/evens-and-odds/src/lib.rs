//! <https://www.codewars.com/kata/583ade15666df5a64e000058/train/rust>

use digital::prelude::*;

pub fn evens_and_odds(n: u64) -> String {
    if n.is_multiple_of(2) {
        n.to_string_base2()
    } else {
        n.to_string_base16()
    }
}
