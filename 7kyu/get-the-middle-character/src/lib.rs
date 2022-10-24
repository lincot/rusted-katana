//! <https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn get_middle(s: &str) -> &str {
    let mut char_indices = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        if s.as_bytes()[i] as i8 >= -64 {
            unsafe { char_indices.push_unchecked(i) };
        }
    }

    if char_indices.len() < 3 {
        s
    } else {
        unsafe {
            s.get_unchecked(
                *char_indices.get_unchecked((char_indices.len() - 1) / 2)
                    ..*char_indices.get_unchecked(char_indices.len() / 2 + 1),
            )
        }
    }
}
