//! <https://www.codewars.com/kata/59c3e8c9f5d5e40cab000ca6/train/rust>

#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};
use core::hint::unreachable_unchecked;
use prelude::*;

pub fn add_arrays(arr_a: &[i64], arr_b: &[i64]) -> Vec<i64> {
    fn to_digits(n: i64) -> Vec<i64> {
        if n == 0 {
            return vec![0];
        }
        if n == i64::MIN {
            return vec![-9, 2, 2, 3, 3, 7, 2, 0, 3, 6, 8, 5, 4, 7, 7, 5, 8, 0, 8];
        }

        let (n, negative) = if n < 0 { (-n, true) } else { (n, false) };

        let mut res = Vec::<i64>::with_capacity(19);
        unsafe { res.write_num_unchecked(n, false, true) };

        if negative {
            if res.is_empty() {
                unsafe { unreachable_unchecked() };
            }
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
