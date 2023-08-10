//! <https://www.codewars.com/kata/5aec1ed7de4c7f3517000079/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn first_n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    let mut arr_enumerated = Vec::with_capacity(arr.len());
    unsafe { arr_enumerated.set_len(arr.len()) };
    let mut e_ptr = arr_enumerated.as_mut_ptr();
    for (i, &x) in arr.iter().enumerate() {
        unsafe {
            *e_ptr = (x, i);
            e_ptr = e_ptr.add(1);
        }
    }
    arr_enumerated.select_nth_unstable(n.saturating_sub(1));
    if n < 64 {
        unsafe { arr_enumerated.get_unchecked_mut(..n) }.sort_by_key(|&(_, i)| i);
    } else {
        radsort::sort_by_key(
            unsafe { arr_enumerated.get_unchecked_mut(..n) },
            |&(_, i)| i,
        );
    }
    let mut res = Vec::with_capacity(n);
    unsafe { res.set_len(n) };
    let mut res_ptr = res.as_mut_ptr();
    for &(x, _) in unsafe { arr_enumerated.get_unchecked(..n) } {
        unsafe {
            *res_ptr = x;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
