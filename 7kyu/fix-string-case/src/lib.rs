//! <https://www.codewars.com/kata/5b180e9fedaa564a7000009a/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use unchecked::{ExtendUnchecked, PushUnchecked};

pub fn solve(s: &str) -> String {
    let mut chars = Vec::with_capacity(s.len());
    for c in s.chars() {
        unsafe { chars.push_unchecked((c, c.is_lowercase(), c.is_uppercase())) };
    }

    let lowercase_count = chars.iter().filter(|&&(_, l, _)| l).count();
    let uppercase_count = chars.iter().filter(|&&(_, _, u)| u).count();

    let mut res = String::with_capacity(3 * s.len());

    if uppercase_count > lowercase_count {
        for &(c, _, u) in &chars {
            if u {
                unsafe { res.push_unchecked(c) };
            } else {
                unsafe { res.extend_unchecked(c.to_uppercase()) };
            }
        }
    } else {
        for &(c, l, _) in &chars {
            if l {
                unsafe { res.push_unchecked(c) };
            } else {
                unsafe { res.extend_unchecked(c.to_lowercase()) };
            }
        }
    }

    res
}
