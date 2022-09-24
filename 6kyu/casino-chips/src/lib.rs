//! <https://www.codewars.com/kata/5e0b72d2d772160011133654/train/rust>

#![no_std]

pub fn solve(arr: &[u32; 3]) -> u32 {
    let [a, b, c] = arr;

    let sum = a + b + c;
    let max = *a.max(b).max(c);
    let sum_others = sum - max;

    if sum_others < max {
        sum_others
    } else {
        sum / 2
    }
}
