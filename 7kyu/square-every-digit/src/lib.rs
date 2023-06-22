//! <https://www.codewars.com/kata/546e2562b03326a88e000020/train/rust>

#![no_std]

pub const fn square_digits(mut num: u64) -> u64 {
    let mut res = 0;
    let mut mul = 1;
    while num != 0 {
        let m = if num % 10 < 4 { 10 } else { 100 };
        res += (num % 10).pow(2) * mul;
        mul *= m;
        num /= 10;
    }
    res
}
