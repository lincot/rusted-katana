//! <https://www.codewars.com/kata/5a71939d373c2e634200008e/train/rust>

#![feature(slice_swap_unchecked)]

use unchecked_std::prelude::*;

pub fn solve(s: &str) -> String {
    let mut chars = Vec::with_capacity(s.len());
    for c in s.chars() {
        unsafe { chars.push_unchecked(c) };
    }

    let mut i = 0;
    let mut j = chars.len() - 1;
    while i < j {
        while *unsafe { chars.get_unchecked(i) } == ' ' {
            i += 1;
        }
        while *unsafe { chars.get_unchecked(j) } == ' ' {
            j -= 1;
        }
        if i < j {
            unsafe { chars.swap_unchecked(i, j) };
        }
        i += 1;
        j -= 1;
    }

    let mut res = String::with_capacity(s.len());
    for c in chars {
        unsafe { res.push_unchecked(c) };
    }
    res
}
