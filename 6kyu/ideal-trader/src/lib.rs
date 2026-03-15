//! <https://www.codewars.com/kata/610ab162bd1be70025d72261/train/rust>

pub fn ideal_trader(prices: &[f64]) -> f64 {
    prices
        .array_windows()
        .map(|[a, b]| (b / a).max(1.))
        .product()
}
