//! <https://www.codewars.com/kata/54b42f9314d9229fd6000d9c/train/rust>

use char_to_lower::to_lower;
use unchecked_std::prelude::*;

pub fn duplicate_encode(word: &str) -> String {
    if word.is_ascii() {
        return unsafe { duplicate_encode_ascii(word) };
    }
    let mut chars = Vec::with_capacity(word.len());
    let mut res_len = 0;
    for ch in word.chars().map(to_lower) {
        if let Some((_, count)) = chars.iter_mut().find(|&&mut (c, _)| c == ch) {
            *count += 1;
        } else {
            unsafe { chars.push_unchecked((ch, 0usize)) };
        }
        res_len += 1;
    }
    let mut res = String::with_capacity(res_len);
    for ch in word.chars().map(to_lower) {
        unsafe {
            let &(_, count) = chars.iter().find(|&&(c, _)| c == ch).unwrap_unchecked();
            if count > 0 {
                res.push_unchecked(')');
            } else {
                res.push_unchecked('(');
            }
        }
    }
    res
}

unsafe fn duplicate_encode_ascii(word: &str) -> String {
    let mut counts = [0usize; 128];
    for b in word.bytes().map(|b| b.to_ascii_lowercase()) {
        *counts.get_unchecked_mut(b as usize) += 1;
    }
    let mut res = String::with_capacity(word.len());
    for b in word.bytes().map(|b| b.to_ascii_lowercase()) {
        if *unsafe { counts.get_unchecked(b as usize) } > 1 {
            res.push_unchecked(')');
        } else {
            res.push_unchecked('(');
        }
    }
    res
}
