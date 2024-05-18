//! <https://www.codewars.com/kata/5a34af40e1ce0eb1f5000036/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn to_csv_text(array: &[Vec<i8>]) -> String {
    unsafe fn push_row(res: &mut String, row: &[i8]) {
        for (i, &x) in row.iter().enumerate() {
            if i != 0 {
                res.push_unchecked(',');
            }
            res.write_num_unchecked(x, 10, false, false);
        }
    }

    let mut res = String::with_capacity(
        array
            .iter()
            .map(|row| (i8::MAX_LEN_BASE10 + 1) * row.len())
            .sum(),
    );
    for (i, row) in array.iter().enumerate() {
        if i != 0 {
            unsafe { res.push_unchecked('\n') };
        }
        unsafe { push_row(&mut res, row) };
    }
    res
}
