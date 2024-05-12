//! <https://www.codewars.com/kata/537ba77315ddd92659000fec/train/rust>

use num_prime::nt_funcs::is_prime64;

pub fn check_goldbach(n: u32) -> Option<(u32, u32)> {
    if n % 2 != 0 || n <= 2 {
        return None;
    }
    if n == 4 {
        return Some((2, 2));
    }

    let ([prime0, prime1], mut value0, mut step) = match n % 3 {
        0 => ([5, 7], 11, 2),
        1 => ([3, 5], 11, 6),
        2 => ([3, 7], 13, 6),
        _ => unreachable!(),
    };

    if is_prime64((n - prime0) as _) {
        return Some((prime0, n - prime0));
    }
    if is_prime64((n - prime1) as _) {
        return Some((prime1, n - prime1));
    }

    let mut value1 = n - value0;
    loop {
        if is_prime64(value0 as _) && is_prime64(value1 as _) {
            return Some((value0, value1));
        }

        value0 += step;
        value1 -= step;

        if step != 6 {
            step ^= 6;
        }
    }
}
