//! <https://www.codewars.com/kata/582c01ad3fd1cc5736000348/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec};

pub fn make_matrix(m: u32, n: u32) -> String {
    unsafe fn push_digit(res: &mut [u8], i: &mut usize, digit: u8) {
        *res.get_unchecked_mut(*i) = digit;
        *i += 2;
    }

    assert!(n % 2 != 0 && n >= 3);

    let digits = [
        (m / 10000) as u8,
        (m / 1000 % 10) as _,
        (m / 100 % 10) as _,
        (m / 10 % 10) as _,
        (m % 10) as _,
    ]
    .map(|x| b'0' + x);
    let mut res = vec![b' '; 2 * n as usize * n as usize];
    let mut index = 0;
    unsafe {
        for i in 0..n / 2 {
            for _ in 0..i {
                push_digit(&mut res, &mut index, digits[3]);
            }
            push_digit(&mut res, &mut index, digits[0]);
            for _ in 0..n - 2 * i - 2 {
                push_digit(&mut res, &mut index, digits[1]);
            }
            push_digit(&mut res, &mut index, digits[0]);
            for _ in 0..i {
                push_digit(&mut res, &mut index, digits[4]);
            }
            *res.get_unchecked_mut(index - 1) = b'\n';
        }
        for _ in 0..n / 2 {
            push_digit(&mut res, &mut index, digits[3]);
        }
        push_digit(&mut res, &mut index, digits[0]);
        for _ in 0..n / 2 {
            push_digit(&mut res, &mut index, digits[4]);
        }
        *res.get_unchecked_mut(index - 1) = b'\n';
        for i in (0..n / 2).rev() {
            for _ in 0..i {
                push_digit(&mut res, &mut index, digits[3]);
            }
            push_digit(&mut res, &mut index, digits[0]);
            for _ in 0..n - 2 * i - 2 {
                push_digit(&mut res, &mut index, digits[2]);
            }
            push_digit(&mut res, &mut index, digits[0]);
            for _ in 0..i {
                push_digit(&mut res, &mut index, digits[4]);
            }
            *res.get_unchecked_mut(index - 1) = b'\n';
        }
        res.set_len(res.len() - 1);
        String::from_utf8_unchecked(res)
    }
}
