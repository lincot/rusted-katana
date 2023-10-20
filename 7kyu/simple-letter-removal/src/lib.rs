//! <https://www.codewars.com/kata/5b728f801db5cec7320000c7/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn solve(s: &str, k: usize) -> String {
    if s.len() <= k {
        return String::new();
    }
    assert!(s.is_ascii());
    let mut bytes: Vec<_> = s.bytes().enumerate().collect();
    if bytes.len() <= 20 {
        bytes.sort_by_key(|&(_, b)| b);
    } else {
        radsort::sort_by_key(&mut bytes, |&(_, b)| b);
    }
    let taken_bytes = unsafe { bytes.get_unchecked_mut(k..) };
    if taken_bytes.len() < 256 {
        taken_bytes.sort_unstable_by_key(|&(i, _)| i);
    } else {
        radsort::sort_by_key(taken_bytes, |&(i, _)| i);
    }
    let res = taken_bytes.iter().map(|&(_, b)| b).collect();
    unsafe { String::from_utf8_unchecked(res) }
}
