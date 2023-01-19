//! <https://www.codewars.com/kata/57a083a57cb1f31db7000028/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn powers_of_two(n: u8) -> Vec<u128> {
    (0..=n).map(|i| 1 << i).collect()
}
