//! <https://www.codewars.com/kata/58f8a3a27a5c28d92e000144/train/rust>

#![no_std]

pub fn first_non_consecutive(arr: &[i32]) -> Option<i32> {
    let mut size = arr.len();

    let first = arr.first()?;

    let mut left = 0;
    let mut right = size;

    while left < right {
        let mid = left + size / 2;

        if *unsafe { arr.get_unchecked(mid) } == mid as i32 + first {
            left = mid + 1;
        } else {
            right = mid;
        }

        size = right - left;
    }

    arr.get(left).copied()
}
