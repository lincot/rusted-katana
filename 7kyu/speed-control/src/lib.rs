//! <https://www.codewars.com/kata/56484848ba95170a8000004d/train/rust>

#![feature(array_windows)]

pub fn gps(s: i32, x: Vec<f64>) -> i32 {
    (3600. * x.array_windows().map(|[a, b]| b - a).fold(0., f64::max) / s as f64) as _
}
