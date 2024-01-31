//! <https://www.codewars.com/kata/5d95b7644a336600271f52ba/train/rust>

use libm::{cos, sin};

pub fn crusoe(n: i32, mut d: f64, ang: i32, distmult: f64, angmult: f64) -> (f64, f64) {
    let mut ang = (ang as f64).to_radians();

    let mut x = 0.;
    let mut y = 0.;

    for _ in 0..n {
        x += d * cos(ang);
        y += d * sin(ang);

        d *= distmult;
        ang *= angmult;
    }

    (x, y)
}
