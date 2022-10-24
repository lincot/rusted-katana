//! <https://www.codewars.com/kata/5d23d89906f92a00267bb83d/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn get_order(input: String) -> String {
    let cap = input.len() + input.len() / 4;
    let mut res = String::with_capacity(cap);

    let mut p = 0usize;
    let mut m = 0;
    let mut n = 0;
    let mut f = 0;
    let mut g = 0;
    let mut o = 0;
    let mut u = 0;
    let mut w = 0;

    for b in input.bytes() {
        match b {
            b'p' => p += 1,
            b'm' => m += 1,
            b'n' => n += 1,
            b'f' => f += 1,
            b'g' => g += 1,
            b'o' => o += 1,
            b'u' => u += 1,
            b'w' => w += 1,
            _ => {}
        }
    }

    for _ in 0..u {
        unsafe { res.push_str_unchecked("Burger ") };
    }
    for _ in 0..f {
        unsafe { res.push_str_unchecked("Fries ") };
    }
    for _ in 0..n - 3 * (g - u) - w {
        unsafe { res.push_str_unchecked("Chicken ") };
    }
    for _ in 0..p {
        unsafe { res.push_str_unchecked("Pizza ") };
    }
    for _ in 0..w {
        unsafe { res.push_str_unchecked("Sandwich ") };
    }
    for _ in 0..g - u {
        unsafe { res.push_str_unchecked("Onionrings ") };
    }
    for _ in 0..m {
        unsafe { res.push_str_unchecked("Milkshake ") };
    }
    for _ in 0..o - 2 * (g - u) {
        unsafe { res.push_str_unchecked("Coke ") };
    }
    res.pop();

    res
}
