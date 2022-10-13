//! <https://www.codewars.com/kata/57f759bb664021a30300007d/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn switcheroo(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    unsafe { res.as_mut_vec().set_len(s.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for &b in s.as_bytes() {
        unsafe {
            *res_ptr = if b == b'a' {
                b'b'
            } else if b == b'b' {
                b'a'
            } else {
                b
            };
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
