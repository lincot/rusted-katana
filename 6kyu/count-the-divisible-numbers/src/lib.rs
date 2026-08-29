//! <https://www.codewars.com/kata/55a5c82cd8e9baa49000004c/train/rust>

pub const fn divisible_count(x: u64, y: u64, k: u64) -> u64 {
    (y - x + x % k) / k + x.is_multiple_of(k) as u64
}
