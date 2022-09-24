//! <https://www.codewars.com/kata/5a4d303f880385399b000001/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn strong(n: u64) -> String {
    if [1, 2, 145, 40585].contains(&n) {
        "STRONG!!!!"
    } else {
        "Not Strong !!"
    }
    .into()
}
