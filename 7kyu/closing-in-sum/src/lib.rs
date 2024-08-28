//! <https://www.codewars.com/kata/65126d52a5de2b11c94096d2/train/rust>

use digital::SumDigits;

pub fn closing_in_sum(mut n: u64) -> u32 {
    let mut ones = 0;
    let mut decimal = 1;
    while n > decimal {
        ones += n % 10;
        n /= 10;
        decimal *= 10;
    }

    let tens = n.sum_digits();

    10 * tens + ones as u32
}
