//! <https://www.codewars.com/kata/55d1d6d5955ec6365400006d/train/rust>

pub const fn round_to_next_5(n: i32) -> i32 {
    n - (n - 1).rem_euclid(5) + 4
}
