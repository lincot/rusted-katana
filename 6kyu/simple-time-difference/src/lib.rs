//! <https://www.codewars.com/kata/5b76a34ff71e5de9db0000f2/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use my_prelude::prelude::*;

fn parse_time(time: &str) -> u32 {
    let time = time.as_bytes();
    assert!(time.len() == 5);

    let hours = 10 * (time[0] - b'0') + (time[1] - b'0');
    let minutes = 10 * (time[3] - b'0') + (time[4] - b'0');

    60 * hours as u32 + minutes as u32
}

pub fn solve(arr: &[&str]) -> String {
    assert!(!arr.is_empty());

    let mut parsed_arr = Vec::with_capacity(arr.len());
    unsafe { parsed_arr.set_len(arr.len()) };
    let mut parsed_arr = parsed_arr.into_boxed_slice();
    let mut parsed_arr_ptr = parsed_arr.as_mut_ptr();
    for s in arr {
        unsafe {
            *parsed_arr_ptr = parse_time(s);
            parsed_arr_ptr = parsed_arr_ptr.add(1);
        }
    }
    parsed_arr.sort_unstable();

    let max_diff = parsed_arr
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .max()
        .unwrap_or_default()
        .max(24 * 60 + parsed_arr[0] - parsed_arr[parsed_arr.len() - 1]);

    let hours = (max_diff - 1) / 60;
    let minutes = (max_diff - 1) % 60;
    let mut res = String::with_capacity(3 + 1 + 2);
    unsafe {
        if hours < 10 {
            res.push_unchecked('0');
        }
        res.write_num_unchecked(hours);
        res.push_unchecked(':');
        if minutes < 10 {
            res.push_unchecked('0');
        }
        res.write_num_unchecked(minutes);
    }
    res
}
