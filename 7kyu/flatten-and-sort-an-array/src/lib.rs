//! <https://www.codewars.com/kata/57ee99a16c8df7b02d00045f/train/rust>

use unchecked_std::prelude::*;

pub fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
    let len = arr.iter().map(Vec::len).sum();
    let mut res = Vec::with_capacity(len);
    for a in arr {
        unsafe { res.extend_from_slice_unchecked(a) };
    }
    vqsort_rs::sort(&mut res);
    res
}
