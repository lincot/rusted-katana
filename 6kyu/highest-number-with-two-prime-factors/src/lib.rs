//! <https://www.codewars.com/kata/55f347cfb44b879e1e00000d/train/rust>

use num_bigint::BigUint;

pub fn highest_bi_prime_factor(pp1: u32, pp2: u32, n: BigUint) -> (BigUint, u32, u32) {
    let mut cur = BigUint::from(pp1) * pp2;
    let (limit_pp1, limit_pp2) = (&n / pp1, &n / pp2);
    let (mut k1, mut k2) = (1, 1);
    while cur <= limit_pp1 {
        cur *= pp1;
        k1 += 1;
    }
    let (mut max, mut bestk1, mut bestk2) = (cur.clone(), k1, k2);
    while k1 != 0 {
        cur /= pp1;
        k1 -= 1;
        while cur <= limit_pp2 {
            cur *= pp2;
            k2 += 1;
        }
        if cur > max {
            (max, bestk1, bestk2) = (cur.clone(), k1, k2);
        }
    }
    (max, bestk1, bestk2)
}
