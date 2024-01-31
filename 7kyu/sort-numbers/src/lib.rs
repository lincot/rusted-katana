//! <https://www.codewars.com/kata/5174a4c0f2769dd8b1000003/train/rust>

use vqsort::VqSort;

pub fn sort_numbers(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.to_vec();
    VqSort::sort_ascending(&mut arr);
    arr
}
