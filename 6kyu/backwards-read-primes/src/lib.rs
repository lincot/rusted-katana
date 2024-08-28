//! <https://www.codewars.com/kata/5539fecef69c483c5a000015/train/rust>

use num_prime::nt_funcs::is_prime64;
use unchecked_std::prelude::*;

pub fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut res = Vec::with_capacity(((stop - start) / 2 + 1) as _);

    let (mut start, mut step) = get_next_and_step(start);

    while start <= stop {
        let (rev, log10) = reverse_digits(start);
        let last_digit = rev % 10;

        if last_digit % 2 == 0 || last_digit == 5 {
            start = (last_digit + 1) * unsafe { POWERS_OF_10.get_unchecked(log10 as usize) } + 1;
            (start, step) = get_next_and_step(start);
        } else {
            if start != rev && is_prime64(start) && is_prime64(rev) {
                unsafe { res.push_unchecked(start) };
            }

            start += step;
            step ^= 6;
        }
    }

    res
}

const POWERS_OF_10: [u64; 20] = {
    let mut res = [1; 20];
    let mut i = 1;
    while i < res.len() {
        res[i] = 10 * res[i - 1];
        i += 1;
    }
    res
};

const fn reverse_digits(mut x: u64) -> (u64, u8) {
    const NUMS_100_REVERSED: [u8; 100] = [
        0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 1, 11, 21, 31, 41, 51, 61, 71, 81, 91, 2, 12, 22,
        32, 42, 52, 62, 72, 82, 92, 3, 13, 23, 33, 43, 53, 63, 73, 83, 93, 4, 14, 24, 34, 44, 54,
        64, 74, 84, 94, 5, 15, 25, 35, 45, 55, 65, 75, 85, 95, 6, 16, 26, 36, 46, 56, 66, 76, 86,
        96, 7, 17, 27, 37, 47, 57, 67, 77, 87, 97, 8, 18, 28, 38, 48, 58, 68, 78, 88, 98, 9, 19,
        29, 39, 49, 59, 69, 79, 89, 99,
    ];

    let mut res = 0;
    let mut log10 = u8::MAX;
    while x >= 10 {
        res *= 100;
        let rev = NUMS_100_REVERSED[(x % 100) as usize];
        res += rev as u64;
        x /= 100;
        log10 = log10.wrapping_add(2);
    }
    if x != 0 {
        res *= 10;
        res += x;
        log10 = log10.wrapping_add(1);
    }
    (res, log10)
}

const fn get_next_and_step(n: u64) -> (u64, u64) {
    let r = n % 6;
    let (next_r, step) = if r <= 1 { (1, 4) } else { (5, 2) };
    let next = n + next_r - r;
    (next, step)
}
