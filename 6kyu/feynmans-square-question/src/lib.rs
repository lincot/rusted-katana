//! <https://www.codewars.com/kata/551186edce486caa61000f5c/train/rust>

pub const fn count_squares(n: u32) -> usize {
    let n = n as usize;
    n * (n + 1) * (2 * n + 1) / 6
}
