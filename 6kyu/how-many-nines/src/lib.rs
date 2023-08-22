//! <https://www.codewars.com/kata/5e18743cd3346f003228b604/train/rust>

#![no_std]

use num_bigint::BigInt;
use prelude::*;

pub fn nines(n: BigInt) -> BigInt {
    let n = n.try_into().unwrap();
    nines_u128(n).into()
}

fn nines_u128(n: u128) -> u128 {
    fn to_digits(n: u128) -> heapless::Vec<u8, 39> {
        let mut digits = heapless::Vec::new();
        unsafe { digits.write_num_unchecked(n, false, true) };
        digits
    }

    let digits = to_digits(n);
    let mut res = 0;
    let mut i = digits.len();
    for d in digits {
        if d == 9 {
            res += unsafe { POWERS_OF_9.get_unchecked(i) } - 1;
            break;
        }
        res += unsafe { POWERS_OF_9.get_unchecked(i - 1) } * d as u128;
        i -= 1;
    }
    n - res
}

const POWERS_OF_9: [u128; 39] = {
    let mut res = [1; 39];
    let mut i = 1;
    while i < res.len() {
        res[i] = 9 * res[i - 1];
        i += 1;
    }
    res
};
