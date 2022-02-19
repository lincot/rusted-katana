//! <https://www.codewars.com/kata/56484848ba95170a8000004d/train/rust>

pub fn gps(s: i32, x: Vec<f64>) -> i32 {
    x.windows(2)
        .map(|pair| (3600. * (pair[1] - pair[0]) / s as f64) as _)
        .fold(0, i32::max)
}
