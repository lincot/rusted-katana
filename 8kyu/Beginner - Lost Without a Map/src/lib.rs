//! <https://www.codewars.com/kata/57f781872e3d8ca2a000007e/train/rust>

pub fn maps(values: &[i32]) -> Vec<i32> {
    values.iter().map(|x| 2 * x).collect()
}
