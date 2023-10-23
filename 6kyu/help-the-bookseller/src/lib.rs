//! <https://www.codewars.com/kata/54dc6f5a224c26032800005c/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use digital::{MaxLenBase10, WriteNumUnchecked};
use prelude::*;

pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return String::new();
    }

    let cap = (4 + u64::MAX_LEN_BASE10 + "( : ) - ".len()) * list_cat.len();
    let mut res = String::with_capacity(cap);

    for (i, cat) in list_cat.into_iter().enumerate() {
        let cat = cat.chars().next().unwrap();

        let mut sum = 0;
        for &art in &list_art {
            if art.starts_with(cat) {
                let space_pos = art.bytes().position(|b| b == b' ').unwrap();
                sum += unsafe { art.get_unchecked(space_pos + 1..) }
                    .parse::<u64>()
                    .unwrap();
            }
        }

        unsafe {
            if i != 0 {
                res.push_str_unchecked(" - ");
            }
            res.push_unchecked('(');
            res.push_unchecked(cat);
            res.push_str_unchecked(" : ");
            res.write_num_unchecked(sum, 10, false, false);
            res.push_unchecked(')');
        }
    }

    res
}
