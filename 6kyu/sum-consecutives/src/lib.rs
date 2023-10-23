//! <https://www.codewars.com/kata/55eeddff3f64c954c2000059/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use unchecked::PushUnchecked;

pub type Number = i32;
pub type Numbers = Vec<Number>;

pub fn sum_consecutives(numbers: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(numbers.len());
    let mut numbers = numbers.iter();
    let Some(&(mut prev_number)) = numbers.next() else {
        return res;
    };
    let mut sum = prev_number;
    for &n in numbers {
        if n == prev_number {
            sum += prev_number;
        } else {
            unsafe { res.push_unchecked(sum) };
            prev_number = n;
            sum = prev_number;
        }
    }
    unsafe { res.push_unchecked(sum) };
    res
}
