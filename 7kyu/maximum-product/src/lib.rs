//! <https://www.codewars.com/kata/5a4138acf28b82aa43000117/train/rust>

#![no_std]

pub fn adjacent_elements_product(xs: &[i32]) -> i32 {
    xs.windows(2).map(|x| x[0] * x[1]).max().unwrap()
}
