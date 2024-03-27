//! <https://www.codewars.com/kata/57cfdf34902f6ba3d300001e/train/rust>

use unchecked_core::{PushStrUnchecked, PushUnchecked};

pub fn two_sort(arr: &[&str]) -> String {
    const STARS: &str = "***";

    let min = arr.iter().min().unwrap();
    let mut min_chars = (*min).chars();

    let cap = (1 + STARS.len()) * min.len();
    let mut res = String::with_capacity(cap);

    if let Some(c) = min_chars.next() {
        unsafe { res.push_unchecked(c) };
    }
    for c in min_chars {
        unsafe { res.push_str_unchecked(STARS) };
        unsafe { res.push_unchecked(c) };
    }

    res
}
