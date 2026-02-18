//! <https://www.codewars.com/kata/52f3149496de55aded000410/train/rust>

use digital::SumDigits;

pub fn sum_digits(number: i32) -> i32 {
    number.unsigned_abs().sum_digits() as i32
}
