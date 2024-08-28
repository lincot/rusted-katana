//! <https://www.codewars.com/kata/55e785dfcb59864f200000d9/train/rust>

use core::hint::unreachable_unchecked;

pub fn count_spec_mult(n: u8, max_val: u64) -> u64 {
    DIVISORS.get(n as usize).map_or(0, |&divisor| unsafe {
        if divisor == 0 {
            unreachable_unchecked();
        }
        max_val / divisor
    })
}

const DIVISORS: [u64; 15] = {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut res = [1; 15];
    let mut i = 1;
    while i < res.len() {
        res[i] = primes[i - 1] * res[i - 1];
        i += 1;
    }
    res
};
