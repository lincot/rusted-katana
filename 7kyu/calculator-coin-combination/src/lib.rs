//! <https://www.codewars.com/kata/564d0490e96393fc5c000029/train/rust>

#![no_std]

pub const fn coin_combo(cents: u64) -> [u64; 4] {
    [cents % 5, cents % 25 % 10 / 5, cents % 25 / 10, cents / 25]
}
