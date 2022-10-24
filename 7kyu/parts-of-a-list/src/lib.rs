//! <https://www.codewars.com/kata/56f3a1e899b386da78000732/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use prelude::*;

pub fn part_list(arr: Vec<&str>) -> String {
    match arr.len() {
        0 => return String::new(),
        1 => {
            let mut res = String::with_capacity(2 + arr[0].len());
            unsafe {
                res.push_unchecked('(');
                res.push_str_unchecked(arr[0]);
                res.push_unchecked(')');
                return res;
            }
        }
        _ => {}
    }

    let mut joined =
        String::with_capacity(arr.iter().map(|s| s.len()).sum::<usize>() + arr.len() - 1);
    let mut comma_poses = Vec::with_capacity(arr.len() - 1);
    for s in arr[..arr.len() - 1].iter() {
        unsafe {
            joined.push_str_unchecked(s);
            comma_poses.push_unchecked(joined.len());
            joined.push_unchecked(' ');
        }
    }
    unsafe { joined.push_str_unchecked(arr[arr.len() - 1]) };

    let mut res = String::with_capacity(comma_poses.len() * (joined.len() + 3));
    for comma_pos in comma_poses {
        unsafe {
            res.push_unchecked('(');
            res.push_str_unchecked(joined.get_unchecked(..comma_pos));
            res.push_unchecked(',');
            res.push_str_unchecked(joined.get_unchecked(comma_pos..));
            res.push_unchecked(')');
        }
    }
    res
}
