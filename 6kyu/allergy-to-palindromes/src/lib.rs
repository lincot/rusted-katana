//! <https://www.codewars.com/kata/5b33452ab6989d2407000100/train/rust>

use num_bigint::BigUint;

pub fn count_pal(n: u32) -> (BigUint, BigUint) {
    let pow = BigUint::from(10u8).pow((n - 1) / 2);
    (
        &pow * 9u8,
        pow * if n.is_multiple_of(2) { 20u8 } else { 11u8 } - 2u8,
    )
}
