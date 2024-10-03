//! <https://www.codewars.com/kata/5168bb5dfe9a00b126000018/train/rust>

use unchecked_std::prelude::*;

pub fn solution(phrase: &str) -> String {
    let mut res = String::with_capacity(phrase.len());
    for ch in phrase.chars().rev() {
        unsafe { res.push_unchecked(ch) };
    }
    res
}
