//! <https://www.codewars.com/kata/58068479c27998b11900056e/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

const TWISTED: [u8; 129] = [
    0, 1, 2, 7, 4, 5, 6, 3, 8, 9, 10, 11, 12, 17, 14, 15, 16, 13, 18, 19, 20, 21, 22, 27, 24, 25,
    26, 23, 28, 29, 70, 71, 72, 77, 74, 75, 76, 73, 78, 79, 40, 41, 42, 47, 44, 45, 46, 43, 48, 49,
    50, 51, 52, 57, 54, 55, 56, 53, 58, 59, 60, 61, 62, 67, 64, 65, 66, 63, 68, 69, 30, 31, 32, 37,
    34, 35, 36, 33, 38, 39, 80, 81, 82, 87, 84, 85, 86, 83, 88, 89, 90, 91, 92, 97, 94, 95, 96, 93,
    98, 99, 100, 101, 102, 107, 104, 105, 106, 103, 108, 109, 110, 111, 112, 117, 114, 115, 116,
    113, 118, 119, 120, 121, 122, 127, 124, 125, 126, 123, 128,
];

pub fn sort_twisted_37(list: &[i8]) -> Vec<i8> {
    let mut res = Vec::with_capacity(list.len());
    unsafe { res.set_len(list.len()) };
    let mut ptr = res.as_mut_ptr();
    for &x in list {
        unsafe {
            *ptr = TWISTED[x.unsigned_abs() as usize] as i8 * x.signum();
            ptr = ptr.add(1);
        }
    }
    res.sort_unstable();
    for x in &mut res {
        *x = TWISTED[x.unsigned_abs() as usize] as i8 * x.signum();
    }
    res
}
