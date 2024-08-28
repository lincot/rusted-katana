//! <https://www.codewars.com/kata/59b8614a5227dd64dc000008/train/rust>

use unchecked_std::prelude::*;

pub fn min_distance(n: u32) -> u32 {
    if n % 2 == 0 {
        return 1;
    } else if [3 * 5, 5 * 7, 7 * 9, 9 * 11].iter().any(|x| n % x == 0) {
        return 2;
    }

    let divisors = get_divisors_odd(n);
    let mut prev = 1;
    let mut min = n - 1;
    for x in divisors {
        min = min.min(x - prev);
        prev = x;
    }
    min
}

/// returns sorted divisors of an odd number, except for 1,
/// and returns nothing, if the number is prime
///
/// # Panics
///
/// panics if the number is not odd
fn get_divisors_odd(mut n: u32) -> Vec<u32> {
    assert!(n % 2 == 1);

    let cap = match n.leading_zeros() {
        27 | 28 => 3,  // 15, 21
        26 => 5,       // 45
        25 => 7,       // 105
        24 => 8,       // 225
        23 => 11,      // 315
        22 => 15,      // 945
        21 => 17,      // 1575
        19 | 20 => 23, // 3465, 4725
        18 => 31,      // 10395
        17 => 39,      // 31185
        16 => 47,      // 45045
        15 => 53,      // 121275
        14 => 71,      // 225225
        13 => 79,      // 405405
        12 => 95,      // 675675
        11 => 119,     // 2027025
        10 => 143,     // 3828825
        9 => 159,      // 6891885
        8 => 191,      // 11486475
        7 => 215,      // 26801775
        6 => 255,      // 43648605
        5 => 319,      // 130945815
        4 => 383,      // 218243025
        3 => 431,      // 509233725
        2 => 511,      // 1003917915
        1 => 575,      // 1527701175
        0 => 639,      // 3011753745
        _ => 0,
    };
    let mut res = Vec::with_capacity(cap);

    let mut x = 3;
    let mut n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
    while x <= n_sqrt {
        let len_before = res.len();
        let mut n_changed = false;
        let mut x_pow = 1;

        while unsafe { n.checked_rem(x).unwrap_unchecked() } == 0 {
            x_pow *= x;
            n /= x;
            n_changed = true;
            unsafe { push_unchecked_with_multiples(&mut res, x_pow, len_before) };
        }

        x += 2;
        if n_changed {
            n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
        }
    }

    let len = res.len();
    if len != 0 {
        if n > 1 {
            unsafe { push_unchecked_with_multiples(&mut res, n, len) };
        }

        vqsort_rs::sort(&mut res);
    }

    res
}

unsafe fn push_unchecked_with_multiples(vec: &mut Vec<u32>, x: u32, n_multiples: usize) {
    vec.push_unchecked(x);
    vec.extend_from_within_unchecked(..n_multiples);
    vec.iter_mut().rev().take(n_multiples).for_each(|n| {
        *n *= x;
    });
}
