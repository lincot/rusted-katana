//! <https://www.codewars.com/kata/565abd876ed46506d600000d/train/rust>

use core::f64::consts::PI;

pub fn simpson(n: i32) -> f64 {
    let k = PI / n as f64;
    #[allow(clippy::suboptimal_flops)]
    let t = 2. * k.cos().powi(3) + 1.;
    k * (2. * k).cos().mul_add(3., t) / (3. * k).sin()
}
