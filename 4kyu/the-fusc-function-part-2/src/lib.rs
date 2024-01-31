//! <https://www.codewars.com/kata/57040e445a726387a1001cf7/train/rust>

use num_bigint::BigUint;

pub fn fusc(n: BigUint) -> BigUint {
    let mut a = 1u8.into();
    let mut b = 0u8.into();
    for d in n.iter_u64_digits() {
        for i in 0..u64::BITS {
            if d >> i & 1 == 0 {
                a += &b;
            } else {
                b += &a;
            }
        }
    }
    b
}
