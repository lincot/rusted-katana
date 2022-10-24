//! <https://www.codewars.com/kata/56dbe0e313c2f63be4000b25/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn hor_mirror(s: String) -> String {
    let mut res = String::with_capacity(s.len());
    for (i, line) in s.rsplit('\n').enumerate() {
        unsafe {
            if i != 0 {
                res.push_unchecked('\n');
            }
            res.push_str_unchecked(line);
        }
    }
    res
}

pub fn vert_mirror(s: String) -> String {
    let mut res = String::with_capacity(s.len());
    for (i, line) in s.split('\n').enumerate() {
        unsafe {
            if i != 0 {
                res.push_unchecked('\n');
            }
            res.extend_unchecked(line.chars().rev());
        }
    }
    res
}

pub fn oper(f: fn(String) -> String, s: String) -> String {
    f(s)
}
