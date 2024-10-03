//! <https://www.codewars.com/kata/5a71939d373c2e634200008e/train/rust>

#![feature(slice_swap_unchecked)]

use unchecked_std::prelude::*;

pub fn solve(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(solve_bytes(s.as_bytes())) };
    }

    let mut chars = Vec::with_capacity(s.len());
    for ch in s.chars() {
        unsafe { chars.push_unchecked(ch) };
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
    for ch in chars {
        unsafe { res.push_unchecked(ch) };
    }
    res
}

fn solve_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = s.to_vec();

    let mut i = 0;
    let mut j = res.len() - 1;
    while i < j {
        while *unsafe { res.get_unchecked(i) } == b' ' {
            i += 1;
        }
        while *unsafe { res.get_unchecked(j) } == b' ' {
            j -= 1;
        }
        if i < j {
            unsafe { res.swap_unchecked(i, j) };
        }
        i += 1;
        j -= 1;
    }

    res
}
