//! <https://www.codewars.com/kata/5a7893ef0025e9eb50000013/train/rust>

use vqsort::VqSort;

pub fn max_gap(xs: &[i32]) -> i32 {
    let mut xs = xs.to_vec();
    VqSort::sort_ascending(&mut xs);
    let mut res = 0;
    for i in 1..xs.len() {
        res = res.max(xs[i] - xs[i - 1]);
    }
    res
}
