//! <https://www.codewars.com/kata/5208fc3cb613bc725f000142/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn solution(st: &str, limit: usize) -> String {
    const DOTS: &str = "...";

    let cap = (4 * limit).min(st.len()) + DOTS.len();
    let mut res = String::with_capacity(cap);

    let mut st = st.chars();

    unsafe { res.extend_unchecked(st.by_ref().take(limit)) };

    if st.next().is_some() {
        unsafe { res.push_str_unchecked(DOTS) };
    }

    res
}
