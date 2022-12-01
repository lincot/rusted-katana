//! <https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/rust>

#![no_std]
#![feature(array_windows)]

pub fn find_missing_letter(chars: &[char]) -> char {
    chars
        .array_windows()
        .map(|&[a, b]| (a as u8 + 1, b as u8))
        .find(|&(a, b)| a != b)
        .unwrap()
        .0 as char
}
