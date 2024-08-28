//! <https://www.codewars.com/kata/581ee0db1bbdd04e010002fd/train/rust>

pub fn interp(f: fn(f64) -> f64, l: f64, u: f64, n: i32) -> Vec<f64> {
    let d = (u - l) / n as f64;
    (0..n).map(|i| floor2(f(d.mul_add(i as _, l)))).collect()
}

#[inline]
fn floor2(mut x: f64) -> f64 {
    x *= 100.;
    if x < 0. {
        x -= 1. - 1e-9;
    }
    x as i64 as f64 / 100.
}
