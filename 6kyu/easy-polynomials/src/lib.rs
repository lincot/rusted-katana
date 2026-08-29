//! <https://www.codewars.com/kata/68aeca3ee7a867797c0d4b22/train/rust>

use core::f64::consts::PI;

use libm::{cos, sin};
use num_complex::Complex;

pub fn easy_polynomials(n: i32) -> Complex<f64> {
    if n % 2 == 0 {
        let pi_frac = PI / (n + 1) as f64;
        -cos(pi_frac) - Complex::i() * sin(pi_frac)
    } else {
        (-1.).into()
    }
}
