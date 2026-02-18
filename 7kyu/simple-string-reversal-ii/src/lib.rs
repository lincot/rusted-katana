//! <https://www.codewars.com/kata/5a8d1c82373c2e099d0000ac/train/rust>

use unchecked_std::prelude::*;

pub fn solve(string: &str, a: usize, b: usize) -> String {
    let b = if b < string.len() {
        b + 1
    } else {
        string.len()
    };
    if string.is_ascii() {
        return unsafe { String::from_utf8_unchecked(solve_bytes(string.as_bytes(), a, b)) };
    }

    let mut res = String::with_capacity(string.len());
    let (head, rest) = string.split_at(a);
    unsafe {
        res.push_str_unchecked(head);
        let (mid, tail) = rest.split_at(b - a);
        for ch in mid.chars().rev() {
            res.push_unchecked(ch);
        }
        res.push_str_unchecked(tail);
    }
    res
}

pub fn solve_bytes(string: &[u8], a: usize, b: usize) -> Vec<u8> {
    let mut res = Vec::with_capacity(string.len());
    unsafe {
        res.extend_from_slice_unchecked(&string[..a]);
        for &b in string[a..b].iter().rev() {
            res.push_unchecked(b);
        }
        res.extend_from_slice_unchecked(&string[b..]);
    }
    res
}
