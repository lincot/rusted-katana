//! <https://www.codewars.com/kata/55e6f5e58f7817808e00002e/train/rust>

#![no_std]

pub const fn seven(mut n: i64) -> (i64, i32) {
    let mut steps = 0;

    while n >= 100 {
        n = n / 10 - 2 * (n % 10);
        steps += 1;
    }

    (n, steps)
}
