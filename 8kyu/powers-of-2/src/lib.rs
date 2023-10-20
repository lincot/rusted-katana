//! <https://www.codewars.com/kata/57a083a57cb1f31db7000028/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn powers_of_two(n: u8) -> Vec<u128> {
    const POWERS_OF_2: [u128; 128] = {
        let mut res = [0; 128];
        res[0] = 1;
        let mut i = 1;
        while i < res.len() {
            res[i] = 2 * res[i - 1];
            i += 1;
        }
        res
    };

    POWERS_OF_2[..=n as _].into()
}
