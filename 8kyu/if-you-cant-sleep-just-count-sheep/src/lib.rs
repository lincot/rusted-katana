//! <https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

/// equals to `(1..=n).map(|x| x.to_string().len()).sum::<usize>() as u32`
fn integral_log10(n: u32) -> u32 {
    if n < 10 {
        return n;
    }
    let log = n.ilog10();
    let mut t = 1;
    for _ in 0..log {
        t = 10 * t + 1;
    }
    (log + 1) * (n + 1) - t
}

pub fn count_sheep(n: u32) -> String {
    const SHEEP: &str = " sheep...";
    let mut res = String::with_capacity(integral_log10(n) as usize + SHEEP.len() * n as usize);
    for sheep in 1..=n {
        unsafe {
            res.write_num_unchecked(sheep);
            res.push_str_unchecked(SHEEP);
        }
    }
    res
}
