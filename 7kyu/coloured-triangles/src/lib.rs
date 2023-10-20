//! <https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111/train/rust>

#![no_std]

extern crate alloc;
use alloc::{boxed::Box, string::String};
use core::hint::unreachable_unchecked;

pub fn triangle(row_str: &str) -> String {
    assert!(!row_str.is_empty());

    let mut colors: Box<[_]> = row_str
        .bytes()
        .map(|b| match b {
            b'R' => 0u8,
            b'G' => 1,
            _ => 2,
        })
        .collect();

    for row in 0..colors.len() {
        for i in 0..colors.len() - row - 1 {
            if i + 1 >= colors.len() {
                unsafe { unreachable_unchecked() }
            }
            if colors[i] != colors[i + 1] {
                colors[i] = 3 - (colors[i] + colors[i + 1]);
            }
        }
    }

    match colors[0] {
        0 => "R".into(),
        1 => "G".into(),
        _ => "B".into(),
    }
}
