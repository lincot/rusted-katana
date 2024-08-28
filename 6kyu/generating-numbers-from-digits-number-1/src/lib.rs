//! <https://www.codewars.com/kata/584d05422609c8890f0000be/train/rust>

use num_bigint::BigUint;

pub fn proc_arr(arr: &[char]) -> (BigUint, BigUint, BigUint) {
    let mut digit_counts = [0; 10];
    for &d in arr {
        if d.is_ascii_digit() {
            digit_counts[(d as u8 - b'0') as usize] += 1;
        }
    }

    let mut n_count = factorial(arr.len());
    for d_count in digit_counts {
        if d_count != 0 {
            n_count /= factorial(d_count);
        }
    }

    let mut min = 0u8.into();
    for (digit, &count) in (1u8..).zip(&digit_counts[1..]) {
        for _ in 0..count {
            min *= 10u8;
            min += digit;
        }
    }
    let mut max = 0u8.into();
    for (digit, count) in (0u8..10u8).zip(digit_counts).rev() {
        for _ in 0..count {
            max *= 10u8;
            max += digit;
        }
    }

    (n_count, min, max)
}

fn factorial(n: usize) -> BigUint {
    if n < FIRST_FACTORIALS.len() {
        return FIRST_FACTORIALS[n].into();
    }

    let mut res = FIRST_FACTORIALS.last().copied().unwrap().into();
    for i in FIRST_FACTORIALS.len()..=n {
        res *= i;
    }
    res
}

#[cfg(target_pointer_width = "64")]
const FIRST_FACTORIALS: [usize; 21] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5_040,
    40_320,
    362_880,
    3_628_800,
    39_916_800,
    479_001_600,
    6_227_020_800,
    87_178_291_200,
    1_307_674_368_000,
    20_922_789_888_000,
    355_687_428_096_000,
    6_402_373_705_728_000,
    121_645_100_408_832_000,
    2_432_902_008_176_640_000,
];
#[cfg(target_pointer_width = "32")]
const FIRST_FACTORIALS: [usize; 13] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5_040,
    40_320,
    362_880,
    3_628_800,
    39_916_800,
    479_001_600,
];
#[cfg(target_pointer_width = "16")]
const FIRST_FACTORIALS: [usize; 9] = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320];
