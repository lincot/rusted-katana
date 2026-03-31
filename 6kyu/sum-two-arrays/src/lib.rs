//! <https://www.codewars.com/kata/59c3e8c9f5d5e40cab000ca6/train/rust>

use core::hint::unreachable_unchecked;

use digital::prelude::*;

pub fn add_arrays(arr_a: &[i64], arr_b: &[i64]) -> Vec<i64> {
    fn to_digits(n: i64) -> Vec<i64> {
        let mut res = Vec::<i64>::with_capacity("9223372036854775807".len());
        unsafe { res.write_int_unchecked_with::<Raw, Base10>(n.unsigned_abs()) };

        if res.is_empty() {
            unsafe { unreachable_unchecked() };
        }

        if n < 0 {
            res[0] = -res[0];
        }

        res
    }

    fn from_digits(digits: &[i64]) -> i64 {
        let mut digits = digits.iter();

        let first = *digits.next().unwrap();

        if first < 0 {
            -digits.fold(-first, |acc, d| 10 * acc + d)
        } else {
            digits.fold(first, |acc, d| 10 * acc + d)
        }
    }

    if arr_a.is_empty() {
        return arr_b.to_vec();
    }
    if arr_b.is_empty() {
        return arr_a.to_vec();
    }

    to_digits(from_digits(arr_a) + from_digits(arr_b))
}
