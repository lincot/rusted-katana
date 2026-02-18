//! <https://www.codewars.com/kata/61559bc4ead5b1004f1aba83/train/rust>

use num_bigint::BigUint;

pub fn spiral_sum(mut n: BigUint) -> BigUint {
    n *= &n + 2u8;
    n -= 1u8;
    n >>= 1;
    n
}
