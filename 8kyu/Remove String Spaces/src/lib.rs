//! <https://www.codewars.com/kata/57eae20f5500ad98e50002c5/train/rust>

#![feature(drain_filter)]

pub fn no_space(mut x: String) -> String {
    let vec = unsafe { x.as_mut_vec() };
    vec.drain_filter(|&mut c| c == b' ');
    x
}
