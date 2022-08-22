//! <https://www.codewars.com/kata/5a61a846cadebf9738000076/train/rust>

use my_prelude::prelude::*;

pub fn peak(arr: &[u32]) -> Option<usize> {
    let mut sums = Vec::with_capacity(arr.len());
    let mut sum = 0;

    for n in arr {
        unsafe { sums.push_unchecked(sum) };
        sum += n;
    }

    if sums.len() != arr.len() {
        unsafe { core::hint::unreachable_unchecked() };
    }

    #[allow(clippy::manual_find)]
    for i in 0..sums.len() {
        if sums[i] == sum - arr[i] - sums[i] {
            return Some(i);
        }
    }
    None
}