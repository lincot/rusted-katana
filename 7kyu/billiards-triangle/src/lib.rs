//! <https://www.codewars.com/kata/5bb3e299484fcd5dbb002912/train/rust>

pub fn pyramid(balls: u16) -> u16 {
    unsafe { ((8.0f64.mul_add(balls as _, 1.).sqrt() - 1.) / 2.).to_int_unchecked() }
}
