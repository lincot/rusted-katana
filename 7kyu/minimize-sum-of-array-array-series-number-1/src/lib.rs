//! <https://www.codewars.com/kata/5a523566b3bfa84c2e00010b/train/rust>

#![no_std]

extern crate alloc;

pub fn min_sum(xs: &[u64]) -> u64 {
    let mut xs = xs.to_vec();
    xs.sort_unstable();

    let mut res = 0;
    for i in 0..xs.len() / 2 {
        res += unsafe { xs.get_unchecked(i) * xs.get_unchecked(xs.len() - i - 1) };
    }
    res
}
