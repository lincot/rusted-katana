//! <https://www.codewars.com/kata/56e3cd1d93c3d940e50006a4/train/rust>

use core::cmp::Reverse;

pub fn make_valley(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort_unstable_by_key(|&v| Reverse(v));

    let len = arr.len();
    let mut res = Vec::with_capacity(len);

    for i in 0..len / 2 {
        unsafe {
            *res.get_unchecked_mut(i) = *arr.get_unchecked(i * 2);
            *res.get_unchecked_mut(len - 1 - i) = *arr.get_unchecked(i * 2 + 1);
        }
    }

    if len % 2 == 1 {
        unsafe {
            *res.get_unchecked_mut(len / 2) = *arr.get_unchecked(len - 1);
        }
    }

    unsafe { res.set_len(len) };

    res
}
