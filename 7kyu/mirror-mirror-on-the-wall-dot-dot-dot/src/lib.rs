//! <https://www.codewars.com/kata/5f55ecd770692e001484af7d/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn mirror(list: &[i32]) -> Vec<i32> {
    if list.len() <= 1 {
        return list.into();
    }

    let mut res = Vec::with_capacity(2 * list.len() - 1);

    unsafe { res.extend_from_slice_unchecked(list) };
    res.sort_unstable();

    for i in (0..res.len() - 1).rev() {
        unsafe {
            res.push_unchecked(*res.get_unchecked(i));
        }
    }

    res
}
