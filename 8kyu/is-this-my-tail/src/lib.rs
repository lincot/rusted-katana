//! <https://www.codewars.com/kata/56f695399400f5d9ef000af5/train/rust>

#![no_std]

pub fn correct_tail(body: &str, tail: char) -> bool {
    body.ends_with(tail)
}
