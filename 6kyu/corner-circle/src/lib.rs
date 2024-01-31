//! <https://www.codewars.com/kata/5898761a9c700939ee000011/train/rust>

use core::f64::consts::SQRT_2;

pub fn corner_circle(r: f64) -> f64 {
    let c = (SQRT_2 - 1.).powi(2);
    (100. * c * r).round() / 100.
}
