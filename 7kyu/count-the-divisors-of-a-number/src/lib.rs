//! <https://www.codewars.com/kata/542c0f198e077084c0000c2e/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

pub fn divisors(mut n: u32) -> u32 {
    // 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 29 > u32::MAX
    let mut prime_pows_incremented = heapless::Vec::<u8, 8>::new();

    let pow_of_2 = n.trailing_zeros();
    n >>= pow_of_2;

    let mut x = 3;
    let mut n_sqrt = unsafe { sqrtf64(n as _).to_int_unchecked() };
    while x <= n_sqrt {
        let mut n_changed = false;
        let mut p = 1;

        while unsafe { n.checked_rem(x).unwrap_unchecked() } == 0 {
            n /= x;
            n_changed = true;
            p += 1;
        }

        if p != 1 {
            unsafe { prime_pows_incremented.push_unchecked(p) };
        }

        x += 2;
        if n_changed {
            n_sqrt = unsafe { sqrtf64(n as _).to_int_unchecked() };
        }
    }

    let mut res = (pow_of_2 + 1) * if n == 1 { 1 } else { 2 };
    for p in prime_pows_incremented {
        res *= p as u32;
    }
    res
}
