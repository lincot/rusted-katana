//! <https://www.codewars.com/kata/55eea63119278d571d00006a/train/rust>

#![no_std]

extern crate alloc;

fn my_binary_search(arr: &[usize]) -> usize {
    let mut size = arr.len();
    let mut left = 0;
    let mut right = size;

    while left < right {
        let mid = left + size / 2;

        if *(unsafe { arr.get_unchecked(mid) }) == mid {
            left = mid + 1;
        } else {
            right = mid;
        }

        size = right - left;
    }

    left
}

pub fn next_id(ids: &[usize]) -> usize {
    let mut ids = ids.to_vec();
    ids.sort_unstable();
    ids.dedup();
    my_binary_search(&ids)
}
