//! <https://www.codewars.com/kata/586c7cd3b98de02ef60001ab/train/rust>

use char_to_lower::to_lower;
use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::{hash_map::Entry, HashMap};
use num_bigint::BigUint;
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn uniq_count(s: &str) -> BigUint {
    if s.bytes().all(|b| b.is_ascii_alphabetic()) {
        return unsafe { uniq_count_ascii(s) };
    }

    let mut freq_map = FxHashMap::with_capacity_and_hasher(s.len(), Default::default());
    for c in s.chars().map(to_lower) {
        if freq_map.len() == freq_map.capacity() {
            unsafe { unreachable_unchecked() };
        }
        match freq_map.entry(c) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1usize);
            }
        }
    }

    let mut denom = BigUint::from(1u8);
    for &count in freq_map.values() {
        denom *= factorial(count);
    }
    factorial(s.len()) / denom
}

unsafe fn uniq_count_ascii(s: &str) -> BigUint {
    let mut frequencies = [0; 26];
    for b in s.bytes() {
        let i = if b >= b'a' { b - b'a' } else { b - b'A' } as usize;
        *frequencies.get_unchecked_mut(i) += 1;
    }

    let mut res = factorial(s.len());
    for count in frequencies {
        if count != 0 {
            res /= factorial(count);
        }
    }
    res
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
