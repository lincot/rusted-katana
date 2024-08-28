//! <https://www.codewars.com/kata/62ca07aaedc75c88fb95ee2f/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn ascend_descend(length: usize, minimum: i32, maximum: i32) -> String {
    if length == 0 || maximum < minimum {
        return String::new();
    }

    let mut res = String::with_capacity(length + i32::MAX_LEN_BASE10);

    for x in minimum..=maximum {
        unsafe {
            res.write_num_unchecked(x, 10, false, false);
            if res.len() >= length {
                res.as_mut_vec().truncate(length);
                return res;
            }
        }
    }
    for x in (minimum..maximum).skip(1).rev() {
        unsafe {
            res.write_num_unchecked(x, 10, false, false);
            if res.len() >= length {
                res.as_mut_vec().truncate(length);
                return res;
            }
        }
    }

    let mut full_cycles_len = res.len();
    while res.len() < length {
        let len = res.len();
        unsafe {
            res.as_mut_vec()
                .extend_from_within_unchecked(..full_cycles_len.min(length - len));
        }
        full_cycles_len *= 2;
    }

    res
}
