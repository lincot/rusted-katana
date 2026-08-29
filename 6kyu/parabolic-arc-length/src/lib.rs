//! <https://www.codewars.com/kata/562e274ceca15ca6e70000d3/train/rust>

pub fn len_curve(n: i32) -> f64 {
    let mut res = 0.;
    let mut x_prev = 0.;
    let mut y_prev = 0.;
    let step = 1. / n as f64;
    #[expect(clippy::imprecise_flops)]
    for _ in 0..n {
        let x = x_prev + step;
        let y = x * x;
        res += ((x - x_prev).powi(2) + (y - y_prev).powi(2)).sqrt();

        x_prev = x;
        y_prev = y;
    }
    res
}
