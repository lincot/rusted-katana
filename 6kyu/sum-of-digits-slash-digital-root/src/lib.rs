//! <https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust>

pub const fn digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}
