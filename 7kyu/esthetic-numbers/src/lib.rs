//! <https://www.codewars.com/kata/6523a71df7666800170a1954/train/rust>

use unchecked::PushUnchecked;

pub fn esthetic(num: u32) -> Vec<u8> {
    let mut res = Vec::with_capacity(9);

    for base in 2..=10 {
        let mut num = num;
        let mut prev_digit = num % base;
        num /= base;
        let mut push = true;

        while num != 0 {
            let digit = num % base;
            num /= base;

            if digit.abs_diff(prev_digit) != 1 {
                push = false;
                break;
            }

            prev_digit = digit;
        }

        if push {
            unsafe { res.push_unchecked(base as _) };
        }
    }

    res
}
