//! <https://www.codewars.com/kata/5d98b6b38b0f6c001a461198/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

const B0: &str = "10";
const B1: &str = "11";
const B2: &str = "0110";
const B3: &str = "0111";
const B4: &str = "001100";
const B5: &str = "001101";
const B6: &str = "001110";
const B7: &str = "001111";
const B8: &str = "00011000";
const B9: &str = "00011001";

pub fn code(s: &str) -> String {
    let mut res = String::with_capacity(8 * s.len());
    for b in s.bytes() {
        unsafe {
            match b {
                b'0' => res.push_str_unchecked(B0),
                b'1' => res.push_str_unchecked(B1),
                b'2' => res.push_str_unchecked(B2),
                b'3' => res.push_str_unchecked(B3),
                b'4' => res.push_str_unchecked(B4),
                b'5' => res.push_str_unchecked(B5),
                b'6' => res.push_str_unchecked(B6),
                b'7' => res.push_str_unchecked(B7),
                b'8' => res.push_str_unchecked(B8),
                _ => res.push_str_unchecked(B9),
            }
        }
    }
    res
}

pub fn decode(s: &str) -> String {
    let mut res = String::with_capacity(s.len() / 2);
    let mut i = 0;
    unsafe {
        while i < s.len() {
            if s.get_unchecked(i..).starts_with(B0) {
                res.push_unchecked('0');
                i += B0.len();
            } else if s.get_unchecked(i..).starts_with(B1) {
                res.push_unchecked('1');
                i += B1.len();
            } else if s.get_unchecked(i..).starts_with(B2) {
                res.push_unchecked('2');
                i += B2.len();
            } else if s.get_unchecked(i..).starts_with(B3) {
                res.push_unchecked('3');
                i += B3.len();
            } else if s.get_unchecked(i..).starts_with(B4) {
                res.push_unchecked('4');
                i += B4.len();
            } else if s.get_unchecked(i..).starts_with(B5) {
                res.push_unchecked('5');
                i += B5.len();
            } else if s.get_unchecked(i..).starts_with(B6) {
                res.push_unchecked('6');
                i += B6.len();
            } else if s.get_unchecked(i..).starts_with(B7) {
                res.push_unchecked('7');
                i += B7.len();
            } else if s.get_unchecked(i..).starts_with(B8) {
                res.push_unchecked('8');
                i += B8.len();
            } else if s.get_unchecked(i..).starts_with(B9) {
                res.push_unchecked('9');
                i += B9.len();
            } else {
                panic!();
            }
        }
    }
    res
}
