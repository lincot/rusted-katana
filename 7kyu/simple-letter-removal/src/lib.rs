//! <https://www.codewars.com/kata/5b728f801db5cec7320000c7/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::mem::{forget, size_of};

pub fn solve(s: &str, k: usize) -> String {
    if s.len() <= k {
        return String::new();
    }
    let mut bytes = Vec::with_capacity(s.len());
    unsafe { bytes.set_len(s.len()) };
    let mut bytes_ptr = bytes.as_mut_ptr();
    for p in s.bytes().enumerate() {
        unsafe {
            assert!(p.1.is_ascii());
            *bytes_ptr = p;
            bytes_ptr = bytes_ptr.add(1);
        }
    }
    bytes.sort_by_key(|&(_, b)| b);
    let taken_bytes = unsafe { bytes.get_unchecked_mut(k..) };
    taken_bytes.sort_unstable_by_key(|&(i, _)| i);

    let mut taken_bytes_ptr = taken_bytes.as_mut_ptr();
    let mut bytes_u8_ptr = bytes.as_mut_ptr().cast();
    for _ in 0..bytes.len() - k {
        unsafe {
            *bytes_u8_ptr = (*taken_bytes_ptr).1;
            bytes_u8_ptr = bytes_u8_ptr.add(1);
            taken_bytes_ptr = taken_bytes_ptr.add(1);
        }
    }

    let res = unsafe {
        String::from_raw_parts(
            bytes.as_mut_ptr().cast(),
            bytes.len() - k,
            size_of::<(usize, u8)>() * bytes.capacity(),
        )
    };
    forget(bytes);
    res
}
