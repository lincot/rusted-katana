//! <https://www.codewars.com/kata/597ef546ee48603f7a000057/train/rust>

#![no_std]

pub fn max_profit(quotes: &[u32]) -> u32 {
    quotes
        .iter()
        .rev()
        .fold((0, 0u32), |(res, max), &quote| {
            (res + max.saturating_sub(quote), max.max(quote))
        })
        .0
}
