//! <https://www.codewars.com/kata/586ec0b8d098206cce001141/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn inverse_slice<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    let a = a.min(input.len());
    let b = b.min(input.len());
    if b <= a {
        return input.to_vec();
    }

    let mut res = Vec::with_capacity(a + input.len() - b);
    // TODO: unchecked
    res.extend_from_slice(&input[..a]);
    res.extend_from_slice(&input[b..]);
    res
}
