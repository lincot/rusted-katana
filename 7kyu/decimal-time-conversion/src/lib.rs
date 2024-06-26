//! <https://www.codewars.com/kata/6397b0d461067e0030d1315e/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn string_to_industrial(time: &str) -> f64 {
    let colon = time.as_bytes().iter().position(|&b| b == b':').unwrap();
    let hours: u32 = unsafe { time.get_unchecked(..colon) }.parse().unwrap();
    let minutes = 10 * (time.as_bytes()[colon + 1] - b'0') + time.as_bytes()[colon + 2] - b'0';
    to_industrial(60 * hours + minutes as u32)
}

pub fn to_industrial(time: u32) -> f64 {
    ((time * 100 + 30) / 60) as f64 / 100.
}

pub fn to_normal(time: f64) -> String {
    let mut res = String::with_capacity(u32::MAX_LEN_BASE10 + 1 + 2);
    unsafe {
        res.write_num_unchecked(time as u32, 10, false, false);
        res.push_unchecked(':');
        let minutes = (time - time as u32 as f64).mul_add(60., 0.5) as u8;
        res.as_mut_vec().push_unchecked(minutes / 10 + b'0');
        res.as_mut_vec().push_unchecked(minutes % 10 + b'0');
    }
    res
}
