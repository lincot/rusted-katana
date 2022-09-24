//! <https://www.codewars.com/kata/5bb3e299484fcd5dbb002912/train/rust>

#![no_std]

use libm::{fma, sqrt};

pub fn pyramid(balls: u16) -> u16 {
    ((sqrt(fma(8.0f64, balls as f64, 1.)) - 1.) / 2.) as _
}
