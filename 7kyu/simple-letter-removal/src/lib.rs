//! <https://www.codewars.com/kata/5b728f801db5cec7320000c7/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn solve(s: &str, k: usize) -> String {
    if s.len() <= k {
        return String::new();
    }
    assert!(s.is_ascii());
    let mut bytes = Vec::with_capacity(s.len());
    unsafe { bytes.set_len(s.len()) };
    for (b, p) in bytes.iter_mut().zip(s.bytes().enumerate()) {
        *b = p;
    }
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

    let mut res = Vec::with_capacity(s.len() - k);
    unsafe { res.set_len(s.len() - k) };
    let mut res_ptr = res.as_mut_ptr();
    for (_, x) in taken_bytes {
        unsafe {
            *res_ptr = *x;
            res_ptr = res_ptr.add(1);
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
