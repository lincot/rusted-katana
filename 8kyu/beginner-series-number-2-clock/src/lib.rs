//! <https://www.codewars.com/kata/55f9bca8ecaa9eac7100004a/train/rust>

pub const fn past(h: i32, m: i32, s: i32) -> i32 {
    ((h * 60 + m) * 60 + s) * 1000
}
