//! <https://www.codewars.com/kata/59c053f070a3b7d19100007e/train/rust>

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    #[allow(clippy::imprecise_flops)]
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

pub fn distance_from_line(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    if a == b {
        dist(a, c)
    } else {
        (a.0 - b.0)
            .mul_add(b.1 - c.1, (c.0 - b.0) * (a.1 - b.1))
            .abs()
            / dist(a, b)
    }
}
