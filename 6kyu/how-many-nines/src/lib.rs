//! <https://www.codewars.com/kata/5e18743cd3346f003228b604/train/rust>

use digital::NumToString;
use num_bigint::BigInt;

pub fn nines(n: BigInt) -> BigInt {
    let n = n.try_into().unwrap();
    nines_u128(n).into()
}

fn nines_u128(n: u128) -> u128 {
    let digits = n.to_heapless_string(false, true).into_bytes();
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
