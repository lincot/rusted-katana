//! <https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d/train/rust>

#![no_std]

pub fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}
