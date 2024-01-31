//! <https://www.codewars.com/kata/5b06c990908b7eea73000069/train/rust>

use num_complex::Complex;

pub fn f(z: Complex<f64>, eps: f64) -> i32 {
    let norm = z.norm();
    if norm >= 1. {
        -1
    } else {
        eps.log(norm) as _
    }
}
