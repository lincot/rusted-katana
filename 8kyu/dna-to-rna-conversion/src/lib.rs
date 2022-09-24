//! <https://www.codewars.com/kata/5556282156230d0e5e000089/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn dna_to_rna(dna: &str) -> String {
    let mut res = String::with_capacity(dna.len());
    unsafe { res.as_mut_vec().set_len(dna.len()) }
    let mut res_ptr = res.as_mut_ptr();
    for &b in dna.as_bytes() {
        unsafe {
            *res_ptr = if b == b'T' { b'U' } else { b };
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
