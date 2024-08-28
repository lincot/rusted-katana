//! <https://www.codewars.com/kata/55ffb44050558fdb200000a4/train/rust>

use digital::SumDigits;

pub fn sum_dig_nth_term(init_val: u32, pattern: &[u32], nth: usize) -> u32 {
    if nth == 0 || pattern.is_empty() {
        return init_val;
    }

    let nth = nth - 1;
    let q = (nth / pattern.len()) as u32;
    let r = nth % pattern.len();

    (init_val + (q + 1) * pattern[..r].iter().sum::<u32>() + q * pattern[r..].iter().sum::<u32>())
        .sum_digits()
}
