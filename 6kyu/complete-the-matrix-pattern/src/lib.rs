//! <https://www.codewars.com/kata/582c01ad3fd1cc5736000348/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn make_matrix(m: u32, n: u32) -> String {
    unsafe fn push_digit(ptr: &mut *mut u8, digit: u8) {
        **ptr = digit;
        *ptr = ptr.add(2);
    }

    assert!(n % 2 != 0 && n >= 3);

    let digits = [
        b'0' + (m / 10000) as u8,
        b'0' + (m / 1000 % 10) as u8,
        b'0' + (m / 100 % 10) as u8,
        b'0' + (m / 10 % 10) as u8,
        b'0' + (m % 10) as u8,
    ];
    let cap = 2 * n as usize * n as usize;
    let mut res = Vec::with_capacity(cap);
    let mut res_ptr = res.as_mut_ptr();
    unsafe {
        res.set_len(cap - 1);
        for _ in 0..res.len() {
            *res_ptr = b' ';
            res_ptr = res_ptr.add(1);
        }
        let mut res_ptr = res.as_mut_ptr();
        for i in 0..n / 2 {
            for _ in 0..i {
                push_digit(&mut res_ptr, digits[3]);
            }
            push_digit(&mut res_ptr, digits[0]);
            for _ in 0..n - 2 * i - 2 {
                push_digit(&mut res_ptr, digits[1]);
            }
            push_digit(&mut res_ptr, digits[0]);
            for _ in 0..i {
                push_digit(&mut res_ptr, digits[4]);
            }
            *res_ptr.sub(1) = b'\n';
        }
        for _ in 0..n / 2 {
            push_digit(&mut res_ptr, digits[3]);
        }
        push_digit(&mut res_ptr, digits[0]);
        for _ in 0..n / 2 {
            push_digit(&mut res_ptr, digits[4]);
        }
        *res_ptr.sub(1) = b'\n';
        for i in (0..n / 2).rev() {
            for _ in 0..i {
                push_digit(&mut res_ptr, digits[3]);
            }
            push_digit(&mut res_ptr, digits[0]);
            for _ in 0..n - 2 * i - 2 {
                push_digit(&mut res_ptr, digits[2]);
            }
            push_digit(&mut res_ptr, digits[0]);
            for _ in 0..i {
                push_digit(&mut res_ptr, digits[4]);
            }
            *res_ptr.sub(1) = b'\n';
        }
        String::from_utf8_unchecked(res)
    }
}
