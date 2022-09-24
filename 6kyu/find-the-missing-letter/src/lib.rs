//! <https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/rust>

#![no_std]

pub fn find_missing_letter(chars: &[char]) -> char {
    chars
        .windows(2)
        .map(|w| (w[0] as u8 + 1, w[1] as u8))
        .find(|&w| w.0 != w.1)
        .unwrap()
        .0 as char
}
