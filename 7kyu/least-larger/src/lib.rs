//! <https://www.codewars.com/kata/5f8341f6d030dc002a69d7e4/train/rust>

#![no_std]

pub fn least_larger(a: &[i32], i: usize) -> Option<usize> {
    let ai = a[i];
    let mut res = None;
    for (i, &x) in a.iter().enumerate() {
        if x > ai && res.map_or(true, |res| x < *unsafe { a.get_unchecked(res) }) {
            res = Some(i);
        }
    }
    res
}
