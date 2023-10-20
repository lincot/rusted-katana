//! <https://www.codewars.com/kata/5556282156230d0e5e000089/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn dna_to_rna(dna: &str) -> String {
    let res = dna
        .as_bytes()
        .iter()
        .map(|&b| if b == b'T' { b'U' } else { b })
        .collect();
    unsafe { String::from_utf8_unchecked(res) }
}
