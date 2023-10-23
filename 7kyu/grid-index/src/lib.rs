//! <https://www.codewars.com/kata/5f5802bf4c2cc4001a6f859e/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use unchecked::PushUnchecked;

pub fn grid_index(grid: &[Vec<char>], indices: &[usize]) -> String {
    let mut res = String::with_capacity(4 * indices.len());
    for i in indices {
        let i = i - 1;
        unsafe { res.push_unchecked(grid[i / grid.len()][i % grid.len()]) };
    }
    res
}
