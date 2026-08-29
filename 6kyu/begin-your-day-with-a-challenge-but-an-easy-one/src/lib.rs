//! <https://www.codewars.com/kata/5873b2010565844b9100026d/train/rust>

use core::{hint::unreachable_unchecked, iter::repeat_n};

pub fn one_two_three(n: u32) -> [String; 2] {
    if n == 0 {
        return ["0".into(), "0".into()];
    }

    let len = ((n / 9) + !n.is_multiple_of(9) as u32) as usize;
    if len == 0 {
        unsafe { unreachable_unchecked() };
    }
    let mut first = vec![b'9'; len];
    if !n.is_multiple_of(9) {
        *first.last_mut().unwrap() = b'0' + (n % 9) as u8;
    }

    let second = repeat_n(b'1', n as _).collect();

    [first, second].map(|v| unsafe { String::from_utf8_unchecked(v) })
}
