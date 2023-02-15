//! <https://www.codewars.com/kata/5a54e796b3bfa8932c0000ed/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn jumping_number(mut n: u64) -> String {
    let mut prev = (n % 10) as u8;
    n /= 10;

    while n != 0 {
        let d = (n % 10) as u8;
        if d != prev + 1 && d + 1 != prev {
            return "Not!!".into();
        }

        prev = d;
        n /= 10;
    }

    "Jumping!!".into()
}
