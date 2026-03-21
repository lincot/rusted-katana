//! <https://www.codewars.com/kata/566543703c72200f0b0000c9/train/rust>

pub fn epidemic(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32 {
    let (mut s, mut i, mut res) = (s0 as f64, i0 as f64, i0 as f64);
    let d = tm as f64 / n as f64;
    for _ in 0..n {
        let infected = d * b * s * i;
        s -= infected;
        i += (d * a).mul_add(-i, infected);
        res = res.max(i);
    }
    res as _
}
