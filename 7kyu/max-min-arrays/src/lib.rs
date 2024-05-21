//! <https://www.codewars.com/kata/5a090c4e697598d0b9000004/train/rust>

use unchecked_std::prelude::*;
use vqsort::VqSort;

pub fn solve(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.to_vec();
    VqSort::sort(&mut arr);
    let mut res = Vec::with_capacity(arr.len());
    let mut it = arr.into_iter();

    while let Some(x) = it.next_back() {
        unsafe { res.push_unchecked(x) };
        if let Some(x) = it.next() {
            unsafe { res.push_unchecked(x) };
        }
    }

    res
}
