//! <https://www.codewars.com/kata/52f5424d0531259cfc000d04/train/rust>

#![no_std]

pub fn sort_by_bit(list: &[u8]) -> u32 {
    list.iter().map(|x| 1 << x).sum()
}
