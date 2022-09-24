//! <https://www.codewars.com/kata/59a8570b570190d313000037/train/rust>

#![no_std]

pub const fn sum_cubes(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}
