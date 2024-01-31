//! <https://www.codewars.com/kata/559b8e46fa060b2c6a0000bf/train/rust>

pub fn diagonal(n: u32, p: u32) -> u64 {
    // binom
    let (mut a, mut b) = (1., 1.);
    for i in 1..p + 2 {
        a *= (n + 2) as f64 - i as f64;
        b *= i as f64;
    }
    (a / b) as _
}
