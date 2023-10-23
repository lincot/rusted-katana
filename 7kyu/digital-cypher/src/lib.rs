//! <https://www.codewars.com/kata/592e830e043b99888600002d/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use digital::NumToString;
use unchecked::PushUnchecked;

pub fn encode(msg: String, n: i32) -> Vec<i32> {
    let digits = (n as u32).to_heapless_string(false, false).into_bytes();

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
