//! <https://www.codewars.com/kata/544aed4c4a30184e960010f4/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let divisors = get_divisors(integer);

    if divisors.is_empty() {
        let mut res = String::with_capacity(u32::MAX_LEN_BASE10 + " is prime".len());
        unsafe {
            res.write_num_unchecked(integer, 10, false, false);
            res.push_str_unchecked(" is prime");
        }
        Err(res)
    } else {
        Ok(divisors)
    }
}

fn get_divisors(mut n: u32) -> Vec<u32> {
    let cap = match n.leading_zeros() {
        30 => 1,   // 2
        29 => 3,   // 6
        28 => 5,   // 12
        27 => 7,   // 24
        26 => 11,  // 60
        25 => 15,  // 120
        24 => 19,  // 240
        23 => 23,  // 360
        22 => 31,  // 840
        21 => 39,  // 1680
        20 => 47,  // 2520
        19 => 63,  // 7560
        18 => 79,  // 15120
        17 => 95,  // 27720
        16 => 119, // 55440
        15 => 143, // 110880
        14 => 167, // 221760
        13 => 199, // 498960
        12 => 239, // 720720
        11 => 287, // 1441440
        10 => 359, // 3603600
        9 => 431,  // 7207200
        8 => 503,  // 14414400
        7 => 599,  // 32432400
        6 => 719,  // 61261200
        5 => 863,  // 122522400
        4 => 1007, // 245044800
        3 => 1151, // 367567200
        2 => 1343, // 735134400
        1 => 1599, // 2095133040
        0 => 1919, // 3491888400
        _ => 0,
    };
    let mut res = Vec::with_capacity(cap);

    let pow_of_2 = n.trailing_zeros();
    for pow in 0..pow_of_2 {
        unsafe { res.push_unchecked(2 << pow) };
    }
    n >>= pow_of_2;

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
        res.pop();
    }

    res
}

unsafe fn push_unchecked_with_multiples(vec: &mut Vec<u32>, x: u32, n_multiples: usize) {
    vec.push_unchecked(x);
    vec.extend_from_within_unchecked(0..n_multiples);
    vec.iter_mut().rev().take(n_multiples).for_each(|n| {
        *n *= x;
    });
}
