//! <https://www.codewars.com/kata/64fbfe2618692c2018ebbddb/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut f = true;
    list.iter()
        .map(|&s| {
            if s == "flick" {
                f = !f;
            }
            f
        })
        .collect()
}
