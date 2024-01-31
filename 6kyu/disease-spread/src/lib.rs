//! <https://www.codewars.com/kata/566543703c72200f0b0000c9/train/rust>

pub fn epidemic(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32 {
    let (mut s, mut i, mut res) = (s0 as f64, i0 as f64, i0 as f64);
    let d = tm as f64 / n as f64;
    for _ in 0..n {
        (s, i) = (
            (d * b * s).mul_add(-i, s),
            d.mul_add((b * s).mul_add(i, -a * i), i),
        );
        res = res.max(i);
    }
    res as _
}
