//! <https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/train/rust>

#![feature(int_log)]

use std::fmt::Write;

/// equals to `(1..=n).map(|x| x.to_string().len()).sum::<usize>() as u32`
fn log10_range_sum(n: u32) -> u32 {
    if n < 10 {
        return n;
    }

    let log = n.log10();

    let mut t = 1;

    for _ in 0..log {
        t = 10 * t + 1;
    }

    (log + 1) * (n + 1) - t
}

pub fn count_sheep(n: u32) -> String {
    let cap = (log10_range_sum(n) + 9 * n) as _;
    let mut murmur = String::with_capacity(cap);

    (1..=n).for_each(|sheep| {
        write!(&mut murmur, "{sheep} sheep...").unwrap();
    });

    murmur
}
