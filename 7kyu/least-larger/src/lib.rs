//! <https://www.codewars.com/kata/5f8341f6d030dc002a69d7e4/train/rust>

pub fn least_larger(xs: &[i32], i: usize) -> Option<usize> {
    let threshold = xs[i];
    xs.iter()
        .enumerate()
        .filter(|(_, &x)| x > threshold)
        .min_by_key(|(_, &x)| x)
        .map(|(j, _)| j)
}
