//! <https://www.codewars.com/kata/545af3d185166a3dec001190/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::mem::{transmute, MaybeUninit};

pub fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    let len = if n == 0 || arr.len() < n {
        0
    } else {
        arr.len() - n + 1
    };
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };
    for (i, r) in res.iter_mut().enumerate() {
        *r = MaybeUninit::new(unsafe { arr.get_unchecked(i..i + n) }.to_vec());
    }
    unsafe { transmute(res) }
}
