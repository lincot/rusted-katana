//! <https://www.codewars.com/kata/5b752a42b11814b09c00005d/train/rust>

#![no_std]

pub const fn solve(mut a: usize, mut b: usize) -> (usize, usize) {
    while a > 0 && b > 0 {
        if a >= 2 * b {
            a %= 2 * b;
        } else if b >= 2 * a {
            b %= 2 * a;
        } else {
            break;
        }
    }
    (a, b)
}
