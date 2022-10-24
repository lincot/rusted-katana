//! <https://www.codewars.com/kata/59c804d923dacc6c41000004/train/rust>

#![no_std]
#![feature(core_intrinsics)]

extern crate alloc;
use alloc::string::String;
use core::intrinsics::sqrtf64;
use prelude::*;

pub fn circle(radius: i32) -> String {
    if radius < 0 {
        return String::new();
    }
    if radius == 0 {
        return "\n".into();
    }
    let radius = radius as usize;

    let cap = "█".len() * (2 * radius - 1).pow(2) + "\n".len() * (2 * radius - 1);
    let mut res = String::with_capacity(cap);

    (0..2 * radius - 1).for_each(|row| {
        let dist_to_center = row.max(radius - 1) - row.min(radius - 1);
        let half_width =
            (unsafe { sqrtf64((radius.pow(2) - dist_to_center.pow(2)) as _) } + 0.999_999) as usize;

        unsafe {
            for _ in 0..radius - half_width {
                res.push_unchecked(' ');
            }
            for _ in 0..2 * half_width - 1 {
                res.push_unchecked('█');
            }
            for _ in 0..radius - half_width {
                res.push_unchecked(' ');
            }
            res.push_unchecked('\n');
        }
    });

    res
}
