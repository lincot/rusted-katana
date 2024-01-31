//! <https://www.codewars.com/kata/5f77d62851f6bc0033616bd8/train/rust>

#![feature(array_windows)]

pub fn valid_spacing(s: &str) -> bool {
    let s = s.as_bytes();
    !(s.first() == Some(&b' ')
        || s.last() == Some(&b' ')
        || s.array_windows().any(|pair| pair == b"  "))
}
