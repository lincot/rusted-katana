//! <https://www.codewars.com/kata/57aa218e72292d98d500240f/train/rust>

pub fn heron(a: u32, b: u32, c: u32) -> f64 {
    let s = (a + b + c) as f64 / 2.;
    (s * (s - a as f64) * (s - b as f64) * (s - c as f64)).sqrt()
}
