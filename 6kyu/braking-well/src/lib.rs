//! <https://www.codewars.com/kata/565c0fa6e3a7d39dee000125/train/rust>

const G: f64 = 9.81;

pub fn dist(v: f64, mu: f64) -> f64 {
    let v = v * (1000. / 3600.);
    v * (v / (2. * mu * G) + 1.)
}

#[expect(clippy::suboptimal_flops)]
pub fn speed(d: f64, mu: f64) -> f64 {
    ((G * mu * (2. * d + G * mu)).sqrt() - G * mu) * (3600. / 1000.)
}
