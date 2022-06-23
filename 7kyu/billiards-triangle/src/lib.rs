//! <https://www.codewars.com/kata/5bb3e299484fcd5dbb002912/train/rust>

pub fn pyramid(balls: u16) -> u16 {
    ((8.0f64.mul_add(balls as f64, 1.).sqrt() - 1.) / 2.) as _
}
