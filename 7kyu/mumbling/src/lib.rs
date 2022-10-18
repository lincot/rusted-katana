//! <https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn accum(s: &str) -> String {
    let cap = if s.is_empty() {
        return String::new();
    } else {
        s.len() * (s.len() + 1) / 2 + s.len() - 1
    };
    let mut res = String::with_capacity(cap);

    for (i, &b) in s.as_bytes().iter().enumerate() {
        if i != 0 {
            unsafe { res.push_unchecked('-') };
        }
        match b {
            b'a'..=b'z' => {
                unsafe { res.as_mut_vec().push_unchecked(b - (b'a' - b'A')) };
                for _ in 0..i {
                    unsafe { res.as_mut_vec().push_unchecked(b) };
                }
            }
            b'A'..=b'Z' => {
                unsafe { res.as_mut_vec().push_unchecked(b) };
                for _ in 0..i {
                    unsafe { res.as_mut_vec().push_unchecked(b + (b'a' - b'A')) };
                }
            }
            _ => panic!(),
        }
    }

    res
}
