//! <https://www.codewars.com/kata/56b5afb4ed1f6d5fb0000991/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use unchecked::{ExtendFromSliceUnchecked, PushUnchecked};

pub fn revrot(s: &str, c: usize) -> String {
    if c == 0 {
        return String::new();
    }

    let s = s.as_bytes();
    assert!(s.iter().all(u8::is_ascii_digit));

    let mut res = Vec::with_capacity(s.len());
    let mut end = c;
    while end <= s.len() {
        let chunk = unsafe { s.get_unchecked(end - c..end) };
        if chunk.iter().fold(0, |acc, &x| x.wrapping_add(acc)) % 2 == 0 {
            for &n in chunk.iter().rev() {
                unsafe { res.push_unchecked(n) };
            }
        } else {
            unsafe {
                res.extend_from_slice_unchecked(&chunk[1..]);
                res.push_unchecked(chunk[0]);
            }
        }

        end += c;
    }
    unsafe { String::from_utf8_unchecked(res) }
}
