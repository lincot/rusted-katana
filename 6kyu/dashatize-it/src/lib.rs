//! <https://www.codewars.com/kata/58223370aef9fc03fd000071/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use core::hint::unreachable_unchecked;
use digital::WriteNumUnchecked;
use prelude::*;

pub fn dashatize(n: i64) -> String {
    fn to_digits(n: u64) -> heapless::Vec<u8, 19> {
        let mut digits = heapless::Vec::new();
        unsafe { digits.write_num_unchecked(n, 10, false, false) };
        if digits.is_empty() {
            unsafe { unreachable_unchecked() };
        }
        digits
    }

    let digits = to_digits(n.unsigned_abs());

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
