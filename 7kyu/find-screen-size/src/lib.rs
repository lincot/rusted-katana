//! <https://www.codewars.com/kata/5bbd279c8f8bbd5ee500000f/train/rust>

use digital::prelude::*;
use unchecked_std::prelude::*;

pub fn find_screen_height(width: u64, ratio: &str) -> String {
    let (vertical, horizontal) = ratio.split_once(':').unwrap();

    let vertical: u64 = vertical.parse().unwrap();
    let horizontal: u64 = horizontal.parse().unwrap();

    let height = width * horizontal / vertical;

    let mut res = String::with_capacity(u64::MAX_LEN_BASE10 + 1 + u64::MAX_LEN_BASE10);
    unsafe {
        res.write_int_unchecked(width);
        res.push_unchecked('x');
        res.write_int_unchecked(height);
    }
    res
}
