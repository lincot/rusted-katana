//! <https://www.codewars.com/kata/54bf85e3d5b56c7a05000cf9/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use digital::WriteNumUnchecked;
use prelude::*;

pub fn number(lines: &[&str]) -> Vec<String> {
    let mut res = Vec::with_capacity(lines.len());
    for (line_number, line) in (1usize..).zip(lines) {
        unsafe {
            let mut numbered_line = String::with_capacity(USIZE_MAX_LEN + 2 + line.len());
            numbered_line.write_num_unchecked(line_number, 10, false, false);
            numbered_line.push_str_unchecked(": ");
            numbered_line.push_str_unchecked(line);
            res.push_unchecked(numbered_line);
        }
    }
    res
}
