//! <https://www.codewars.com/kata/563b74ddd19a3ad462000054/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use prelude::*;

pub fn stringy(size: usize) -> String {
    let mut res = Vec::with_capacity(size);
    unsafe {
        for _ in 0..size / 2 {
            res.extend_from_slice_unchecked(b"10");
        }
        if size % 2 == 1 {
            res.push_unchecked(b'1');
        }
        String::from_utf8_unchecked(res)
    }
}
