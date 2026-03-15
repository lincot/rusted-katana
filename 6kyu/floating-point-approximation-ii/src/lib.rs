//! <https://www.codewars.com/kata/581ee0db1bbdd04e010002fd/train/rust>

#[expect(clippy::suboptimal_flops)]
pub fn interp(f: fn(f64) -> f64, l: f64, u: f64, n: i32) -> Vec<f64> {
    let d = (u - l) / n as f64;
    (0..n)
        .map(|i| (f(d * i as f64 + l) * 100.).floor() / 100.)
        .collect()
}
