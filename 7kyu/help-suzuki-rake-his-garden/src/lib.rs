//! <https://www.codewars.com/kata/571c1e847beb0a8f8900153d/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn rake_garden(garden: &str) -> String {
    let mut res = String::with_capacity("gravel".len() * garden.len());
    let mut word_start = 0;
    for (i, &b) in garden.as_bytes().iter().enumerate() {
        if b == b' ' {
            if unsafe { garden.get_unchecked(word_start..i) } == "rock" {
                unsafe { res.push_str_unchecked("rock ") };
            } else {
                unsafe { res.push_str_unchecked("gravel ") };
            }
            word_start = i + 1;
        }
    }
    if unsafe { garden.get_unchecked(word_start..) } == "rock" {
        unsafe { res.push_str_unchecked("rock") };
    } else {
        unsafe { res.push_str_unchecked("gravel") };
    }
    res
}
