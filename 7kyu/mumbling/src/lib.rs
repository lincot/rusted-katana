//! <https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust>

use my_prelude::prelude::*;

pub fn accum(s: &str) -> String {
    let cap = if s.is_empty() {
        return String::new();
    } else {
        s.len() * (s.len() + 1) / 2 + s.len() - 1
    };
    let mut res = Vec::with_capacity(cap);

    let mut s = s.bytes();

    match s.next().unwrap() {
        b @ b'a'..=b'z' => {
            unsafe { res.push_unchecked(b - (b'a' - b'A')) };
        }
        b @ b'A'..=b'Z' => {
            unsafe { res.push_unchecked(b) };
        }
        _ => panic!(),
    }
    for (i, b) in (1..).zip(s) {
        unsafe { res.push_unchecked(b'-') };
        match b {
            b'a'..=b'z' => {
                unsafe { res.push_unchecked(b - (b'a' - b'A')) };
                for _ in 0..i {
                    unsafe { res.push_unchecked(b) };
                }
            }
            b'A'..=b'Z' => {
                unsafe { res.push_unchecked(b) };
                for _ in 0..i {
                    unsafe { res.push_unchecked(b + (b'a' - b'A')) };
                }
            }
            _ => panic!(),
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
