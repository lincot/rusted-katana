//! <https://www.codewars.com/kata/56cca888a9d0f25985000036/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::{hint::unreachable_unchecked, intrinsics::sqrtf32};
use num_bigint::BigUint;

/// checks if `x` is prime || `x` is divisible by 2 or 3 || `x` <= 1
fn is_prime_with_condition(x: u16) -> bool {
    let mut divisor = 5;
    let mut step = 2;
    while divisor <= unsafe { sqrtf32(x as _).to_int_unchecked() } {
        if divisor == 0 {
            unsafe { unreachable_unchecked() };
        }
        if x % divisor == 0 {
            return false;
        }

        divisor += step;
        step ^= 6;
    }

    true
}

pub fn candies_to_buy(amount_of_kids_invited: u16) -> BigUint {
    const POWERS_OF_3: [u32; 12] = {
        let mut res = [0; 12];
        res[1] = 1;
        let mut i = 2;
        while i < res.len() {
            res[i] = 3 * res[i - 1];
            i += 1;
        }
        res
    };

    fn my_binary_search(arr: &[u32], n: u32) -> u32 {
        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = (left + right) / 2;

            if *unsafe { arr.get_unchecked(mid) } <= n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        *unsafe { arr.get_unchecked(left - 1) }
    }

    let mut res = (my_binary_search(&POWERS_OF_3, amount_of_kids_invited as _)
        << (u16::BITS - 1 - amount_of_kids_invited.leading_zeros()))
    .into();

    let mut n = 5;
    let mut step = 2;

    while n <= amount_of_kids_invited {
        if is_prime_with_condition(n) {
            let mut r = n as u32;
            res *= loop {
                let next = r * n as u32;
                if next > amount_of_kids_invited as _ {
                    break r;
                }
                r = next;
            }
        }

        n += step;
        step ^= 6;
    }

    res
}
