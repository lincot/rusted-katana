//! <https://www.codewars.com/kata/5a4138acf28b82aa43000117/train/rust>

#![feature(array_windows)]

pub fn adjacent_elements_product(xs: &[i32]) -> i32 {
    xs.array_windows().map(|[a, b]| a * b).max().unwrap()
}
