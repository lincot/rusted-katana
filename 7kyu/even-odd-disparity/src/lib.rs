//! <https://www.codewars.com/kata/59c62f1bdcc40560a2000060/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn solve(v: &[String]) -> i32 {
    let mut balance = 0;

    for x in v {
        if let Some(b) = x.bytes().last() {
            if b"02468".contains(&b) {
                balance += 1;
            } else if b"13579".contains(&b) {
                balance -= 1;
            }
        }
    }

    balance
}
