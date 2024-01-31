//! <https://www.codewars.com/kata/585c50e75d0930e6a7000336/train/rust>

use num_integer::gcd;

pub fn are_coprime(n: u8, m: u8) -> bool {
    gcd(n, m) == 1
}
