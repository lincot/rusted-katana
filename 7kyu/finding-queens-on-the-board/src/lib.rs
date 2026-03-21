//! <https://www.codewars.com/kata/64060d8ab2dd990058b7f8ee/train/rust>

pub const fn queens(n: i64) -> i64 {
    if n <= 2 { 0 } else { (n - 2) * (n - 1) }
}
