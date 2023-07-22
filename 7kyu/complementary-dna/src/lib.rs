//! <https://www.codewars.com/kata/554e4a2f232cdd87d9000038/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn dna_strand(dna: &str) -> String {
    let mut res = String::with_capacity(dna.len());
    unsafe { res.as_mut_vec().set_len(dna.len()) };
    for (r, b) in unsafe { res.as_mut_vec() }.iter_mut().zip(dna.as_bytes()) {
        *r = match b {
            b'A' => b'T',
            b'T' => b'A',
            b'C' => b'G',
            _ => b'C',
        };
    }
    res
}
