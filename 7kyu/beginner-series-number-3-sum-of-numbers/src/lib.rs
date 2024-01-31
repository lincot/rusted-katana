//! <https://www.codewars.com/kata/55f2b110f61eb01779000053/train/rust>

pub const fn get_sum(a: i64, b: i64) -> i64 {
    (a + b) * ((a - b).abs() + 1) / 2
}
