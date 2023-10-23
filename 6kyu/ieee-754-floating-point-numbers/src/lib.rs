//! <https://www.codewars.com/kata/5efcaedf95d7110017896ced/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use unchecked::PushUnchecked;

pub fn f32_to_ieee_754(n: f32) -> String {
    let n = n.to_bits();
    let mut res = Vec::with_capacity(34);
    unsafe {
        res.push_unchecked(b'0' + (n >> 31 != 0) as u8);
        res.push_unchecked(b' ');
        for i in (23..31).rev() {
            res.push_unchecked(b'0' + ((n & 1 << i) != 0) as u8);
        }
        res.push_unchecked(b' ');
        for i in (0..23).rev() {
            res.push_unchecked(b'0' + ((n & 1 << i) != 0) as u8);
        }
        String::from_utf8_unchecked(res)
    }
}

pub fn f64_to_ieee_754(n: f64) -> String {
    let n = n.to_bits();
    let mut res = Vec::with_capacity(66);
    unsafe {
        res.push_unchecked(b'0' + (n >> 63 != 0) as u8);
        res.push_unchecked(b' ');
        for i in (52..63).rev() {
            res.push_unchecked(b'0' + ((n & 1 << i) != 0) as u8);
        }
        res.push_unchecked(b' ');
        for i in (0..52).rev() {
            res.push_unchecked(b'0' + ((n & 1 << i) != 0) as u8);
        }
        String::from_utf8_unchecked(res)
    }
}
