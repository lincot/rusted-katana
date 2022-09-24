//! <https://www.codewars.com/kata/554e4a2f232cdd87d9000038/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn dna_strand(dna: &str) -> String {
    let mut res = String::with_capacity(dna.len());
    unsafe { res.as_mut_vec().set_len(dna.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for b in dna.as_bytes() {
        unsafe {
            *res_ptr = match b {
                b'A' => b'T',
                b'T' => b'A',
                b'C' => b'G',
                _ => b'C',
            };
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
