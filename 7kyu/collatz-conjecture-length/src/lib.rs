//! <https://www.codewars.com/kata/54fb963d3fe32351f2000102/train/rust>

#![no_std]

pub const fn collatz(mut n: u64) -> u64 {
    let mut res = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n *= 3;
            n += 1;
        }
        res += 1;
    }
    res
}
