//! <https://www.codewars.com/kata/550527b108b86f700000073f/train/rust>

use core::f64::consts::PI;

pub fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut pi_approx = 0.0;
    let mut numer = 4.0;
    let mut denom = 1;

    loop {
        pi_approx += numer / denom as f64;
        numer = -numer;
        denom += 2;

        if (pi_approx - PI).abs() < epsilon {
            return (denom / 2, rnd10(pi_approx));
        }
    }
}

fn rnd10(f: f64) -> f64 {
    (f * 1e10).round() / 1e10
}
