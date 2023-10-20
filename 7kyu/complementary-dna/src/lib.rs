//! <https://www.codewars.com/kata/554e4a2f232cdd87d9000038/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn dna_strand(dna: &str) -> String {
    let res = dna
        .bytes()
        .map(|b| match b {
            b'A' => b'T',
            b'T' => b'A',
            b'C' => b'G',
            _ => b'C',
        })
        .collect();
    unsafe { String::from_utf8_unchecked(res) }
}
