//! <https://www.codewars.com/kata/56b97b776ffcea598a0006f2/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn bubblesort_once(lst: &[u32]) -> Vec<u32> {
    let mut vec = Vec::with_capacity(lst.len());
    unsafe { vec.set_len(lst.len()) };
    if lst.is_empty() {
        return vec;
    }
    let mut vec_ptr = vec.as_mut_ptr();
    let mut max = lst[0];
    for &x in &lst[1..] {
        if x > max {
            unsafe {
                *vec_ptr = max;
                vec_ptr = vec_ptr.add(1);
            }
            max = x;
        } else {
            unsafe {
                *vec_ptr = x;
                vec_ptr = vec_ptr.add(1);
            }
        }
    }
    unsafe { *vec_ptr = max };
    vec
}
