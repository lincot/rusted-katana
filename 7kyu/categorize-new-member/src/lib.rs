//! <https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa/train/rust>

#![no_std]

extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use prelude::*;

pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut res = Vec::with_capacity(data.len());
    for (age, handicap) in data {
        unsafe {
            res.push_unchecked(if age >= 55 && handicap > 7 {
                "Senior".to_string()
            } else {
                "Open".to_string()
            });
        }
    }
    res
}
