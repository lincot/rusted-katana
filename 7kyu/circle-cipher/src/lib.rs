//! <https://www.codewars.com/kata/634d0723075de3f97a9eb604/train/rust>

use unchecked_std::prelude::*;

pub fn encode(s: String) -> String {
    let mut res = String::with_capacity(s.len());
    let mut s = s.chars();
    while let Some(c) = s.next() {
        unsafe { res.push_unchecked(c) };
        if let Some(c) = s.next_back() {
            unsafe { res.push_unchecked(c) };
        } else {
            break;
        }
    }
    res
}

pub fn decode(s: String) -> String {
    let mut res = String::with_capacity(s.len());
    let mut t = Vec::with_capacity(s.len() / 2 + 1);
    let mut s = s.chars();
    while let Some(c) = s.next() {
        unsafe { res.push_unchecked(c) };
        if let Some(c) = s.next() {
            unsafe { t.push_unchecked(c) };
        } else {
            break;
        }
    }
    for &c in t.iter().rev() {
        unsafe { res.push_unchecked(c) };
    }
    res
}
