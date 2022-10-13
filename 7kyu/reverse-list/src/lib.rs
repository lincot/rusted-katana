//! <https://www.codewars.com/kata/57a04da9e298a7ee43000111/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn reverse_list(lst: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(lst.len());
    unsafe { res.set_len(lst.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for &x in lst.iter().rev() {
        unsafe {
            *res_ptr = x;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
