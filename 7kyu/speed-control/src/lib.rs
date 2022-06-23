//! <https://www.codewars.com/kata/56484848ba95170a8000004d/train/rust>

pub fn gps(s: i32, x: Vec<f64>) -> i32 {
    (3600.
        * x.windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(0., f64::max)
        / s as f64) as _
}
