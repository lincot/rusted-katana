//! <https://www.codewars.com/kata/58223370aef9fc03fd000071/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn dashatize(mut n: i64) -> String {
    fn to_digits(n: i64) -> heapless::Vec<u8, 19> {
        let mut digits = heapless::Vec::new();
        unsafe { digits.write_num_unchecked(n) };
        if digits.is_empty() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        digits
    }

    if n == i64::MIN {
        return "9-22-3-3-7-20-3-68-5-4-7-7-5-808".into();
    }
    if n < 0 {
        n = -n;
    }

    let digits = to_digits(n);

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
