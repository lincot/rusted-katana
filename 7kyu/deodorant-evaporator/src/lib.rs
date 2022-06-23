//! <https://www.codewars.com/kata/5506b230a11c0aeab3000c1f/train/rust>

pub fn evaporator(_content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    ((threshold as f64 * 0.01).log(1.0 - evap_per_day as f64 * 0.01) + 0.999_999_99) as _
}
