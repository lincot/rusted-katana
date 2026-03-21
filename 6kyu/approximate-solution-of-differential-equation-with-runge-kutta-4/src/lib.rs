//! <https://www.codewars.com/kata/60633afe35b4960032fd97f9/train/rust>

pub fn rk4(x0: f64, y0: f64, h: f64, f: fn(f64, f64) -> f64, x1: f64) -> Vec<f64> {
    let mut x = x0;
    let mut y = y0;
    (0..((x1 - x0) / h) as usize + 1)
        .map(|_| {
            let prev_y = y;
            let k1 = h * f(x, y);
            let k2 = h * f(x + h / 2., y + k1 / 2.);
            let k3 = h * f(x + h / 2., y + k2 / 2.);
            let k4 = h * f(x + h, y + k3);
            y = k3.mul_add(2., k2.mul_add(2., k1 + k4)).mul_add(1. / 6., y);
            x += h;
            prev_y
        })
        .collect()
}
