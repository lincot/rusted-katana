//! <https://www.codewars.com/kata/5b7176768adeae9bc9000056/train/rust>

#![no_std]

pub fn index_equals_value(arr: &[i32]) -> i32 {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = (left + right) / 2;
        if mid as i32 <= *unsafe { arr.get_unchecked(mid) } {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left < arr.len() && left as i32 == arr[left] {
        left as i32
    } else {
        -1
    }
}
