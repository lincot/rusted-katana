//! <https://www.codewars.com/kata/5a34af40e1ce0eb1f5000036/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use prelude::*;

pub fn to_csv_text(array: &[Vec<i8>]) -> String {
    unsafe fn push_row(res: &mut String, row: &[i8]) {
        for (i, &x) in row.iter().enumerate() {
            if i != 0 {
                res.push_unchecked(',');
            }
            res.write_num_unchecked(x, false, false);
        }
    }

    let mut res = String::with_capacity(array.iter().map(|row| 5 * row.len()).sum());
    for (i, row) in array.iter().enumerate() {
        if i != 0 {
            unsafe { res.push_unchecked('\n') };
        }
        unsafe { push_row(&mut res, row) };
    }
    res
}
