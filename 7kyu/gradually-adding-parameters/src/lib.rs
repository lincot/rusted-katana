//! <https://www.codewars.com/kata/555b73a81a6285b6ce000047/train/rust>

#![no_std]

pub fn add(args: &[i64]) -> i64 {
    (1..).zip(args).map(|(i, x)| i * x).sum()
}
