//! <https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust>

pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= 0. || bounce <= 0. || bounce >= 1. || window >= h {
        return -1;
    }
    2 * ((window / h).log(bounce) - 1e-9) as i32 + 1
}
