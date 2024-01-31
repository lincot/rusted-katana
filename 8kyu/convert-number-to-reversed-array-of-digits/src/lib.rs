//! <https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust>

use digital::NumToString;

pub fn digitize(n: u64) -> Vec<u8> {
    n.to_string(true, true).into_bytes()
}
