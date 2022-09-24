//! <https://www.codewars.com/kata/5a34af40e1ce0eb1f5000036/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use my_prelude::prelude::*;

pub fn to_csv_text(array: &[Vec<i8>]) -> String {
    #[inline(always)]
    fn push_row(row: &[i8], res: &mut String) {
        let mut row = row.iter();
        if let Some(&x) = row.next() {
            unsafe { res.write_num_unchecked(x) };
        }
        for &x in row {
            unsafe { res.push_unchecked(',') };
            unsafe { res.write_num_unchecked(x) };
        }
    }

    let mut res = String::with_capacity(array.iter().map(|row| 5 * row.len()).sum());
    let mut array = array.iter();
    if let Some(row) = array.next() {
        push_row(row, &mut res);
    }
    for row in array {
        unsafe { res.push_unchecked('\n') };
        push_row(row, &mut res);
    }
    res
}
