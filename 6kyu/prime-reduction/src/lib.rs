//! <https://www.codewars.com/kata/59aa6567485a4d03ff0000ca/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

/// checks if `x` is prime || `x` is divisible by 2 or 3 || `x` <= 1
fn is_prime_with_condition(x: u32) -> bool {
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

const fn sum_of_squared_digits(mut x: u32) -> u32 {
    let mut res = 0;
    while x != 0 {
        res += (x % 10).pow(2);
        x /= 10;
    }
    res
}

const fn reduces_to_one(mut x: u32) -> bool {
    x = sum_of_squared_digits(x);
    while x > 6 {
        if x % 10 == 5 {
            return false;
        }
        x = sum_of_squared_digits(x);
    }
    x == 1
}

pub fn solve(mut a: u32, b: u32) -> usize {
    if a < 7 {
        a = 7;
    }
    let r = a % 6;
    let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2) };
    a += next_r - r;
    let mut res = 0;
    while a < b {
        if is_prime_with_condition(a) && reduces_to_one(a) {
            res += 1;
        }

        a += step;
        step ^= 6;
    }
    res
}
