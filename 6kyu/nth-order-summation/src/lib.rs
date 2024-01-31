//! <https://www.codewars.com/kata/60d5b5cd507957000d08e673/train/rust>

use num_bigint::BigUint;

pub fn s(m: u8, n: BigUint) -> BigUint {
    let mut a = BigUint::from(1u8);
    let mut b = BigUint::from(1u8);
    for i in 0..m {
        a *= i + &n;
        b *= i + 1;
    }
    a / b
}
