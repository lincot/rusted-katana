//! <https://www.codewars.com/kata/565c0fa6e3a7d39dee000125/train/rust>

const G: f64 = 9.81;

pub fn dist(v: f64, mu: f64) -> f64 {
    let v = v * (1000. / 3600.);
    v.mul_add(v / (2. * mu * G), v)
}

pub fn speed(d: f64, mu: f64) -> f64 {
    mu.mul_add(-G, (G * mu * d.mul_add(2., G * mu)).sqrt()) * (3600. / 1000.)
}
