//! <https://www.codewars.com/kata/57ebaa8f7b45ef590c00000c/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn switcher(numbers: Vec<&str>) -> String {
    let mut res = String::with_capacity(numbers.len());
    unsafe { res.as_mut_vec().set_len(numbers.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for s in numbers {
        let s = s.as_bytes();
        let n = match s.len() {
            2 => 10 * (s[0] - b'0') + (s[1] - b'0'),
            1 => s[0] - b'0',
            _ => panic!(),
        } - 1;
        unsafe {
            *res_ptr = b"zyxwvutsrqponmlkjihgfedcba!? "[n as usize];
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
