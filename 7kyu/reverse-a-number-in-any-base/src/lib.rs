//! <https://www.codewars.com/kata/6469e4c905eaefffd44b6504/train/rust>

#![no_std]

pub const fn reversed_number(mut n: u32, b: u32) -> u64 {
    if b == 1 {
        return n as _;
    }
    let mut res = 0;
    while n >= b {
        res *= b as u64;
        res += (n % b) as u64;
        n /= b;
    }
    res + n as u64
}
