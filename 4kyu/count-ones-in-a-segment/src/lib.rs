//! <https://www.codewars.com/kata/596d34df24a04ee1e3000a25/train/rust>

#![no_std]

pub fn count_ones(left: u64, right: u64) -> u64 {
    count_ones_from_zero_to(right) - count_ones_from_zero_to(left - 1)
}

fn count_ones_from_zero_to(n: u64) -> u64 {
    (0..u64::BITS - n.leading_zeros())
        .map(|i| {
            let b = 1 << i;
            if n & b == 0 {
                0
            } else if i == 0 {
                1
            } else {
                1 + (n & (b - 1)) + ((i as u64) << (i - 1))
            }
        })
        .sum()
}
