//! <https://www.codewars.com/kata/5a63948acadebff56f000018/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn max_product(mut lst: Vec<i32>, n_largest_elements: i32) -> i32 {
    let i = lst.len() - n_largest_elements as usize;
    lst.select_nth_unstable(i);
    lst[i..].iter().product()
}
