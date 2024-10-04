//! <https://www.codewars.com/kata/5b40b666dfb4291ad9000049/train/rust>

pub fn solve(a: i32, b: i32, c: i32, alpha: i32, beta: i32, gamma: i32) -> Vec<i32> {
    let (x, y) = [
        (a as f64, 0., (alpha as f64).to_radians().sin_cos()),
        (0., b as f64, (beta as f64).to_radians().sin_cos()),
        (-c as f64, 0., (gamma as f64).to_radians().sin_cos()),
    ]
    .iter()
    .fold((0.0, 0.0), |(x, y), &(a, b, (s, c))| {
        (
            b.mul_add(-s, a.mul_add(c, x)),
            b.mul_add(c, a.mul_add(s, y)),
        )
    });

    let co = x.mul_add(x, y.powi(2)).sqrt();

    let t0c = y.atan2(x).to_degrees();
    let t0c_minutes = t0c.fract() * 60.;
    let t0c_seconds = t0c_minutes.fract() * 60.;

    vec![
        (co + 0.5) as _,
        t0c as _,
        t0c_minutes as _,
        t0c_seconds as _,
    ]
}
