//! <https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust>

use unchecked_std::prelude::*;

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds = Vec::with_capacity(arr.len());
    for &x in arr {
        if x % 2 == 1 {
            unsafe { odds.push_unchecked(x) };
        }
    }
    vqsort_rs::sort(&mut odds);
    let mut odd_i = 0;
    arr.iter()
        .map(|&x| {
            if x % 2 == 1 {
                odd_i += 1;
                unsafe { *odds.get_unchecked(odd_i - 1) }
            } else {
                x
            }
        })
        .collect()
}
