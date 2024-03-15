//! <https://www.codewars.com/kata/610ab162bd1be70025d72261/train/rust>

#![feature(array_windows)]

pub fn ideal_trader(prices: &[f64]) -> f64 {
    prices
        .array_windows()
        .filter(|[a, b]| b > a)
        .map(|[a, b]| b / a)
        .product()
}
