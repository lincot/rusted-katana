//! <https://www.codewars.com/kata/546dba39fa8da224e8000467/train/rust>

use unchecked_std::prelude::*;

pub fn run_length_encoding(s: &str) -> Vec<(usize, char)> {
    if s.is_ascii() {
        return run_length_encoding_bytes(s.as_bytes());
    }

    let mut res = Vec::with_capacity(s.len());
    let mut chars = s.chars();
    let mut count = 1;
    let Some(mut prev) = chars.next() else {
        return res;
    };
    for ch in chars {
        if ch == prev {
            count += 1;
        } else {
            unsafe { res.push_unchecked((count, prev)) };
            (count, prev) = (1, ch);
        }
    }
    unsafe { res.push_unchecked((count, prev)) };
    res
}

fn run_length_encoding_bytes(s: &[u8]) -> Vec<(usize, char)> {
    let mut res = Vec::with_capacity(s.len());
    let mut bytes = s.iter();
    let mut count = 1;
    let Some(&(mut prev)) = bytes.next() else {
        return res;
    };
    for &b in bytes {
        if b == prev {
            count += 1;
        } else {
            unsafe { res.push_unchecked((count, prev as _)) };
            (count, prev) = (1, b);
        }
    }
    unsafe { res.push_unchecked((count, prev as _)) };
    res
}
