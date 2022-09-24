//! <https://www.codewars.com/kata/566fc12495810954b1000030/train/rust>

#![no_std]

pub fn nb_dig(n: i32, d: i32) -> i32 {
    let mut res = (d == 0) as _;

    for mut k in (0..=n).map(|k| k * k) {
        while k != 0 {
            if k % 10 == d {
                res += 1;
            }
            k /= 10;
        }
    }

    res
}
