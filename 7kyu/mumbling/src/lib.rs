//! <https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust>

use unchecked_std::prelude::*;

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
        if b.is_ascii_lowercase() {
            unsafe { res.as_mut_vec().push_unchecked(b.to_ascii_uppercase()) };
            for _ in 0..i {
                unsafe { res.as_mut_vec().push_unchecked(b) };
            }
        } else if b.is_ascii_uppercase() {
            unsafe { res.as_mut_vec().push_unchecked(b) };
            for _ in 0..i {
                unsafe { res.as_mut_vec().push_unchecked(b.to_ascii_lowercase()) };
            }
        } else {
            panic!();
        }
    }
    res
}
