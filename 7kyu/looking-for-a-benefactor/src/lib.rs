//! <https://www.codewars.com/kata/569b5cec755dd3534d00000f/train/rust>

pub fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
    match newavg.mul_add((arr.len() + 1) as _, -arr.iter().sum::<f64>()) {
        n if n > 0. => Some((n + 0.99999) as _),
        _ => None,
    }
}
