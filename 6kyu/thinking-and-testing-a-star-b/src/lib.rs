//! <https://www.codewars.com/kata/5a90c9ecb171012b47000077/train/rust>

use digital::SumDigits;

pub fn test_it(a: u64, b: u64) -> u64 {
    (a.sum_digits() * b.sum_digits()) as _
}
