//! <https://www.codewars.com/kata/58aed2cafab8faca1d000e20/train/rust>

pub fn modified_sum(array: &[i32], power: u32) -> i32 {
    array.iter().map(|x| x.pow(power) - x).sum()
}
