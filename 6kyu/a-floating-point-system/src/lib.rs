//! <https://www.codewars.com/kata/5df754981f177f0032259090/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn mant_exp(a_number: &str, digits_number: i32) -> String {
    let digits_before_dot = a_number.as_bytes().iter().position(|&b| b == b'.').unwrap();
    let mut res = String::with_capacity(digits_number as usize + 1 + 1 + 20);
    if digits_before_dot >= digits_number as usize {
        unsafe {
            res.push_str_unchecked(a_number.get_unchecked(..digits_number as usize));
            res.push_unchecked('P');
            res.write_num_unchecked(digits_before_dot - digits_number as usize);
        }
    } else if digits_before_dot == 1 && *unsafe { a_number.as_bytes().get_unchecked(0) } == b'0' {
        let zeros_after_dot = unsafe { a_number.as_bytes().get_unchecked(digits_before_dot + 1..) }
            .iter()
            .position(|&b| b != b'0')
            .unwrap();
        let nonzero_digit_pos = digits_before_dot + 1 + zeros_after_dot;
        let nonzero_digits = a_number.len() - nonzero_digit_pos;
        if nonzero_digits >= digits_number as usize {
            unsafe {
                res.push_str_unchecked(
                    a_number.get_unchecked(
                        nonzero_digit_pos..nonzero_digit_pos + digits_number as usize,
                    ),
                );
            }
        } else {
            unsafe {
                res.push_str_unchecked(a_number.get_unchecked(nonzero_digit_pos..));
                for _ in 0..digits_number as usize - nonzero_digits {
                    res.push_unchecked('0');
                }
            }
        }
        unsafe {
            res.push_unchecked('P');
            res.push_unchecked('-');
            res.write_num_unchecked(digits_number as usize + zeros_after_dot);
        }
    } else {
        unsafe {
            res.push_str_unchecked(a_number.get_unchecked(..digits_before_dot));
            if digits_number as usize > a_number.len() - 1 {
                res.push_str_unchecked(a_number.get_unchecked(digits_before_dot + 1..));
                for _ in 0..digits_number as usize - (a_number.len() - 1) {
                    res.push_unchecked('0');
                }
            } else {
                res.push_str_unchecked(
                    a_number.get_unchecked(digits_before_dot + 1..=digits_number as usize),
                );
            }
            res.push_unchecked('P');
            res.push_unchecked('-');
            res.write_num_unchecked(digits_number as usize - digits_before_dot);
        }
    }
    res
}
