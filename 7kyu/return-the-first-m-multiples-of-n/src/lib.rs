//! <https://www.codewars.com/kata/593c9175933500f33400003e/train/rust>

pub fn multiples(m: i32, n: f64) -> Vec<f64> {
    (1..=m).map(|i| i as f64 * n).collect()
}
