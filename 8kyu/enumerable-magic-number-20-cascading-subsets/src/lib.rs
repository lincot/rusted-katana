//! <https://www.codewars.com/kata/545af3d185166a3dec001190/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    let len = if n == 0 || arr.len() < n {
        0
    } else {
        arr.len() - n + 1
    };
    let mut res = Vec::with_capacity(len);
    for i in 0..len {
        unsafe {
            res.push_unchecked(arr.get_unchecked(i..i + n).to_vec());
        }
    }
    res
}
