//! <https://www.codewars.com/kata/5d50e3914861a500121e1958/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn add_letters(letters: Vec<char>) -> char {
    const A: u32 = 'a' as u32 - 1;
    const Z: char = 'z';
    match letters.iter().map(|&c| c as u32 - A).sum::<u32>() % (Z as u32 - A) {
        0 => Z,
        i => char::from_u32(i + A).unwrap(),
    }
}
