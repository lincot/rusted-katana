//! <https://www.codewars.com/kata/592e830e043b99888600002d/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use prelude::*;

pub fn encode(msg: String, n: i32) -> Vec<i32> {
    fn to_digits(n: u32) -> heapless::Vec<u8, 10> {
        let mut digits = heapless::Vec::new();
        unsafe { digits.write_num_unchecked(n) };
        digits
    }

    let digits = to_digits(n as u32);

    let mut res = Vec::with_capacity(msg.len());
    let mut i = 0;

    for b in msg.as_bytes() {
        unsafe {
            res.push_unchecked(
                (b.wrapping_add(*digits.get_unchecked(i))
                    .wrapping_sub(b'a' + b'0' - 1)) as _,
            );
        }
        if i == digits.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }

    res
}
