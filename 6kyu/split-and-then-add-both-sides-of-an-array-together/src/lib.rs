//! <https://www.codewars.com/kata/5946a0a64a2c5b596500019a/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::hint::unreachable_unchecked;

pub fn split_and_add(arr: &[u32], n: usize) -> Vec<u32> {
    assert!(!arr.is_empty());
    let mut arr = arr.to_vec();

    for _ in 0..(n as u32).min(usize::BITS - (arr.len() - 1).leading_zeros()) {
        if arr.len() < 2 {
            unsafe { unreachable_unchecked() };
        }

        let d2 = arr.len() / 2;
        let r2 = arr.len() % 2;

        if r2 == 0 {
            for i in 0..d2 {
                if i >= arr.len() {
                    unsafe { unreachable_unchecked() };
                }
                arr[i] += *unsafe { arr.get_unchecked(i + d2) };
            }
        } else {
            let mid = arr[d2];
            for i in (1..=d2).rev() {
                if i >= arr.len() {
                    unsafe { unreachable_unchecked() };
                }
                arr[i] = unsafe { arr.get_unchecked(i - 1) + arr.get_unchecked(i + d2) };
            }
            arr[0] = mid;
        }

        arr.truncate(d2 + r2);
    }

    arr
}
