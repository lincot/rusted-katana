//! <https://www.codewars.com/kata/5e0607115654a900140b3ce3/train/rust>

#![no_std]

pub fn sequence(n: usize) -> i64 {
    const POWERS_OF_3: [i64; 38] = {
        let mut res = [1; 38];
        let mut i = 1;
        while i < res.len() {
            res[i] = 3 * res[i - 1];
            i += 1;
        }
        res
    };
    let mut res = 0;
    for (i, power) in POWERS_OF_3.into_iter().enumerate() {
        if n & (1 << i) != 0 {
            res += power;
        }
    }
    res
}
