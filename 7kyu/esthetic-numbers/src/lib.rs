//! <https://www.codewars.com/kata/6523a71df7666800170a1954/train/rust>

use unchecked_std::prelude::*;
use unroll_lite::unroll;

pub fn esthetic(num: u32) -> Vec<u8> {
    let mut res = Vec::with_capacity(9);

    unroll!(base in 2..11 => {
        let (mut num, mut prev_digit) = (num / base, num % base);
        let mut push = true;
        while num != 0 {
            if (num % base).abs_diff(prev_digit) != 1 {
                push = false;
                break;
            }
            (num, prev_digit) = (num / base, num % base);
        }
        if push {
            unsafe { res.push_unchecked(base as _) };
        }
    });

    res
}
