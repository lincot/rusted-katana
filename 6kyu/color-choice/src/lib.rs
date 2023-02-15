//! <https://www.codewars.com/kata/55be10de92aad5ef28000023/train/rust>

#![no_std]

use core::cmp::Ordering;

pub fn check_choose(m: u64, n: u64) -> i64 {
    let mut x = 1;
    for i in 0..n / 2 {
        x = x * (n - i) / (i + 1);
        match x.cmp(&m) {
            Ordering::Equal => return (i + 1) as _,
            Ordering::Greater => break,
            Ordering::Less => {}
        }
    }
    -1
}
