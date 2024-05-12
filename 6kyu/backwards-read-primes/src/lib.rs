//! <https://www.codewars.com/kata/5539fecef69c483c5a000015/train/rust>

use num_prime::nt_funcs::is_prime64;
use unchecked_core::PushUnchecked;

const POWERS_OF_10: [u64; 20] = {
    let mut res = [1; 20];
    let mut i = 1;
    while i < res.len() {
        res[i] = 10 * res[i - 1];
        i += 1;
    }
    res
};

const fn reverse_digits(mut x: u64) -> (u64, u8, u8) {
    let mut res = 0;
    let mut log10 = u8::MAX;
    let mut last_digit = 0;
    while x != 0 {
        res *= 10;
        last_digit = (x % 10) as _;
        res += last_digit as u64;
        x /= 10;
        log10 = log10.wrapping_add(1);
    }
    (res, log10, last_digit)
}

const fn get_next_and_step(n: u64) -> (u64, u64) {
    let r = n % 6;
    let (next_r, step) = if r <= 1 { (1, 4) } else { (5, 2) };
    let next = n + next_r - r;
    (next, step)
}

pub fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut res = Vec::with_capacity(((stop - start) / 2 + 1) as _);

    let (mut start, mut step) = get_next_and_step(start);

    while start <= stop {
        let (rev, log10, last_digit) = reverse_digits(start);

        if last_digit % 2 == 0 || last_digit == 5 {
            start =
                (last_digit + 1) as u64 * unsafe { POWERS_OF_10.get_unchecked(log10 as usize) } + 1;
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
