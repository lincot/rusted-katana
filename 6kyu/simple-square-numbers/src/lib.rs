//! <https://www.codewars.com/kata/5edc8c53d7cede0032eb6029/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

pub fn solve(n: u64) -> Option<u64> {
    if n == 0 {
        Some(1)
    } else if n % 4 == 2 {
        None
    } else if n % 4 == 0 {
        let m = n / 4;
        (1u64..unsafe { (sqrtf64(m as _) + 1.5).to_int_unchecked() })
            .rev()
            .skip_while(|b| m <= b.pow(2))
            .find(|b| m % b == 0)
            .map(|b| {
                let a = m / b;
                let x = a - b;
                x.pow(2)
            })
    } else {
        (1..unsafe { (sqrtf64(n as _) + 2.5).to_int_unchecked::<u64>() } / 2)
            .rev()
            .map(|i| 2 * i - 1)
            .skip_while(|b| n <= b.pow(2))
            .find(|b| n % b == 0)
            .map(|b| {
                let a = n / b;
                let x = (a - b) / 2;
                x.pow(2)
            })
    }
}
