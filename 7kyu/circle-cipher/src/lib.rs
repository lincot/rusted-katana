//! <https://www.codewars.com/kata/634d0723075de3f97a9eb604/train/rust>

use unchecked_std::prelude::*;

pub fn encode(s: String) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(encode_bytes(s.as_bytes())) };
    }

    let mut res = String::with_capacity(s.len());
    let mut s = s.chars();
    while let Some(ch) = s.next() {
        unsafe { res.push_unchecked(ch) };
        if let Some(ch) = s.next_back() {
            unsafe { res.push_unchecked(ch) };
        } else {
            break;
        }
    }
    res
}

fn encode_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(s.len());
    for i in 0..s.len() / 2 {
        unsafe {
            res.push_unchecked(s[i]);
            res.push_unchecked(*s.get_unchecked(s.len() - 1 - i));
        }
    }
    if s.len() % 2 == 1 {
        unsafe { res.push_unchecked(s[s.len() / 2]) };
    }
    res
}

pub fn decode(s: String) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(decode_bytes(s.as_bytes())) };
    }

    let mut res = String::with_capacity(s.len());
    let mut t = Vec::with_capacity(s.len() / 2 + 1);
    let mut s = s.chars();
    while let Some(ch) = s.next() {
        unsafe { res.push_unchecked(ch) };
        if let Some(ch) = s.next() {
            unsafe { t.push_unchecked(ch) };
        } else {
            break;
        }
    }
    for &ch in t.iter().rev() {
        unsafe { res.push_unchecked(ch) };
    }
    res
}

fn decode_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(s.len());

    let mut i = 0;
    while i < s.len() {
        unsafe {
            res.push_unchecked(s[i]);
        }
        i += 2;
    }

    if s.len() > 1 {
        let mut i = s.len() - 1 - s.len() % 2;
        loop {
            unsafe { res.push_unchecked(*s.get_unchecked(i)) };
            if i == 1 {
                break;
            }
            i -= 2;
        }
    }

    res
}
