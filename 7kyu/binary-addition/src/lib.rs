//! <https://www.codewars.com/kata/551f37452ff852b7bd000139/train/rust>

use digital::NumToString;

pub fn add_binary(a: u64, b: u64) -> String {
    (a + b).to_string_base2(false, false)
}
