//! <https://www.codewars.com/kata/5a2be17aee1aaefe2a000151/train/rust>

pub fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter().sum::<i32>() + ys.iter().sum::<i32>()
}
