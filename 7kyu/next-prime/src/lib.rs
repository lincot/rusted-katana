//! <https://www.codewars.com/kata/58e230e5e24dde0996000070/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

/// checks if `x` is prime || `x` is divisible by 2 or 3 || `x` <= 1
fn is_prime_with_condition(x: u64) -> bool {
    let mut divisor = 5;
    let mut step = 2;
    while divisor <= unsafe { sqrtf64(x as _).to_int_unchecked() } {
        if divisor == 0 {
            unsafe { core::hint::unreachable_unchecked() };
        }
        if x % divisor == 0 {
            return false;
        }

        divisor += step;
        step ^= 6;
    }

    true
}

pub fn next_prime(mut n: u64) -> u64 {
    match n {
        0 | 1 => return 2,
        2 => return 3,
        _ => {}
    }

    let (first_step, mut step) = match n % 6 {
        0 => (1, 4),
        1 => (4, 2),
        2 => (3, 2),
        3 => (2, 2),
        4 => (1, 2),
        5 => (2, 4),
        _ => unreachable!(),
    };
    n += first_step;

    loop {
        if is_prime_with_condition(n) {
            return n;
        }

        n += step;
        step ^= 6;
    }
}
