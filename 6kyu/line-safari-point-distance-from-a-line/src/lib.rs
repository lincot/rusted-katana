//! <https://www.codewars.com/kata/59c053f070a3b7d19100007e/train/rust>

#[expect(clippy::suboptimal_flops)]
pub fn distance_from_line(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    if a == b {
        dist(a, c)
    } else {
        ((b.0 - a.0) * (a.1 - c.1) - (a.0 - c.0) * (b.1 - a.1)).abs() / dist(a, b)
    }
}

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    #[expect(clippy::imprecise_flops)]
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}
