//! <https://www.codewars.com/kata/5ffc226ce1666a002bf023d2/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn solution(mtrx: &[Vec<char>]) -> bool {
    mtrx.iter().any(|row| {
        row.iter()
            .position(|&c| c == '>')
            .map_or(false, |i| row[i + 1..].iter().any(|&c| c == 'x'))
    })
}
