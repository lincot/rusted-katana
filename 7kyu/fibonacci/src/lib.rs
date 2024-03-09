//! <https://www.codewars.com/kata/57a1d5ef7cb1f3db590002af/train/rust>

pub fn fib(n: u32) -> u32 {
    let s5 = 5f64.sqrt();
    let phi = (1. + s5) / 2.;
    let little_phi = (1. - s5) / 2.;
    ((phi.powi(n as _) - little_phi.powi(n as _)) / s5 + 1e-6) as _
}
