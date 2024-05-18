//! <https://www.codewars.com/kata/58223370aef9fc03fd000071/train/rust>

use digital::NumToString;
use unchecked_std::prelude::*;

pub fn dashatize(n: i64) -> String {
    let digits = n
        .unsigned_abs()
        .to_heapless_string(false, false)
        .into_bytes();
    let mut res = String::with_capacity(2 * digits.len());

    let first = digits[0];
    unsafe { res.as_mut_vec().push_unchecked(first) };
    let mut was_odd = first % 2 == 1;

    for &d in &digits[1..] {
        let is_odd = d % 2 == 1;

        if was_odd || is_odd {
            unsafe { res.push_unchecked('-') };
        }
        unsafe { res.as_mut_vec().push_unchecked(d) };

        was_odd = is_odd;
    }

    res
}
