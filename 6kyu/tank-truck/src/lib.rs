//! <https://www.codewars.com/kata/55f3da49e83ca1ddae0000ad/train/rust>

use std::f32::consts::FRAC_PI_4;

pub fn tank_vol(h: i32, d: i32, vt: i32) -> i32 {
    let (h, d, vt) = (h as f32, d as f32, vt as f32);
    let r = d / 2.;
    let l = vt / FRAC_PI_4 / d.powi(2);
    (l * r.powi(2).mul_add(
        ((r - h) / r).acos(),
        -(r - h) * d.mul_add(h, -h.powi(2)).sqrt(),
    )) as _
}
