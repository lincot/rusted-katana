//! <https://www.codewars.com/kata/6512b3775bf8500baea77663/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn gimme_the_letters(sp: &str) -> String {
    let [b, _, e]: [u8; 3] = sp.as_bytes().try_into().unwrap();

    let table = if b.is_ascii_lowercase() {
        b"abcdefghijklmnopqrstuvwxyz"
    } else {
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    };

    unsafe {
        String::from_utf8_unchecked(table[(b - table[0]) as usize..=(e - table[0]) as usize].into())
    }
}
