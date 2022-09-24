//! <https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust>

#![no_std]

pub fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().map(|&x| x.max(0)).sum()
}
