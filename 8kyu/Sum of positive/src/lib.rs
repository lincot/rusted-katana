//! <https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust>

pub fn positive_sum(slice: &[i32]) -> i32 {
    // same assembly as with filter
    slice.iter().map(|&x| x.max(0)).sum()
}
