//! <https://www.codewars.com/kata/617dcb2f242452004a77c653/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

#[allow(clippy::ptr_arg)]
pub fn merge<'a>(xs: &'a Vec<usize>, ys: &'a Vec<usize>) -> Vec<&'a usize> {
    xs.iter().chain(ys).collect()
}
