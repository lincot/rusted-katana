//! <https://www.codewars.com/kata/5616868c81a0f281e500005c/train/rust>

use core::cmp::Reverse;
use unchecked_std::prelude::*;

pub fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    assert!(n != 0);
    if st.is_empty() {
        return "No participants";
    }
    let mut participants = Vec::with_capacity(st.len() + 1);
    let mut start = 0;
    let mut winning_number = 0;
    for (i, b) in st.bytes().enumerate() {
        if b == b',' {
            winning_number *= we[participants.len()];
            unsafe {
                participants.push_unchecked((winning_number, Reverse(st.get_unchecked(start..i))));
            }
            start = i + 1;
            winning_number = 0;
        } else {
            winning_number += (b - if b >= b'a' { b'a' } else { b'A' } + 2) as i32;
        }
    }
    winning_number *= we[participants.len()];
    unsafe { participants.push_unchecked((winning_number, Reverse(st.get_unchecked(start..)))) };
    let len = participants.len();
    if n > len {
        return "Not enough participants";
    }
    participants.select_nth_unstable(len - n).1 .1 .0
}
