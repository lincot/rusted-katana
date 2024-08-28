//! <https://www.codewars.com/kata/58184387d14fc32f2b0012b2/train/rust>

pub fn f(x: f64) -> f64 {
    x / 2. - x.powi(2) / 8. + x.powi(3) / 16.
}
