//! <https://www.codewars.com/kata/5a2fd38b55519ed98f0000ce/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn multi_table(n: u64) -> String {
    let mut repeating = heapless::Vec::<_, { " *  = ".len() + 20 }>::new();
    let repeating = unsafe {
        repeating.extend_from_slice_unchecked(b" * ");
        repeating.write_num_unchecked(n, false, false);
        repeating.extend_from_slice_unchecked(b" = ");
        core::str::from_utf8_unchecked(&repeating)
    };

    let cap = 20 * repeating.len() - 31;
    let mut res = String::with_capacity(cap);
    unsafe {
        res.push_unchecked('1');
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(n, false, false);
        res.push_str_unchecked("\n2");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(2 * n, false, false);
        res.push_str_unchecked("\n3");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(3 * n, false, false);
        res.push_str_unchecked("\n4");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(4 * n, false, false);
        res.push_str_unchecked("\n5");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(5 * n, false, false);
        res.push_str_unchecked("\n6");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(6 * n, false, false);
        res.push_str_unchecked("\n7");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(7 * n, false, false);
        res.push_str_unchecked("\n8");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(8 * n, false, false);
        res.push_str_unchecked("\n9");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(9 * n, false, false);
        res.push_str_unchecked("\n10");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(10 * n, false, false);
    }
    res
}
