//! <https://www.codewars.com/kata/56bdd0aec5dc03d7780010a5/train/rust>

#![no_std]

pub const fn next_higher(n: i32) -> i32 {
    let t = n | (n - 1);
    (t + 1) | ((!t & -!t) - 1) >> (n.trailing_zeros() + 1)
}
