//! <https://www.codewars.com/kata/5552101f47fc5178b1000050/train/rust>

use digital::{CountDigitsBase10, Next2Digits};

pub fn dig_pow(n: i64, p: i32) -> i64 {
    let n_orig = n as u64;
    let mut n = n_orig;
    let mut pow_sum = 0;
    let mut p = p as u32 + n.count_digits_base10() as u32 - 1;

    while let Some([a, b]) = n.next_2_digits(true) {
        pow_sum += (b as u64).pow(p);
        pow_sum += (a as u64).pow(p - 1);
        p -= 2;
    }
    if n != 0 {
        pow_sum += n.pow(p);
    }

    let k = pow_sum / n_orig;
    if n_orig * k == pow_sum {
        k as _
    } else {
        -1
    }
}
