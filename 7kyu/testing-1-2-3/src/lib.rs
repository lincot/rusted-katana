//! <https://www.codewars.com/kata/54bf85e3d5b56c7a05000cf9/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::mem::{transmute, MaybeUninit};
use prelude::*;

pub fn number(lines: &[&str]) -> Vec<String> {
    let mut res = Vec::with_capacity(lines.len());
    unsafe { res.set_len(lines.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for (line_number, line) in (1usize..).zip(lines) {
        unsafe {
            let mut numbered_line = String::with_capacity(line.len() + 22);
            numbered_line.write_num_unchecked(line_number);
            numbered_line.push_str_unchecked(": ");
            numbered_line.push_str_unchecked(line);
            *res_ptr = MaybeUninit::new(numbered_line);
            res_ptr = res_ptr.add(1);
        }
    }
    unsafe { transmute(res) }
}
