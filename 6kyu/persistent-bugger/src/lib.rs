//! <https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec/train/rust>

use digital::prelude::*;

pub fn persistence(mut num: u64) -> u64 {
    let mut res = 0;
    while num > 9 {
        res += 1;

        let mut product = 1;
        while let Some([a, b]) = num.next_2_digits::<Raw>() {
            product *= a as u64 * b as u64;
        }
        if num != 0 {
            product *= num;
        }

        num = product;
    }
    res
}
