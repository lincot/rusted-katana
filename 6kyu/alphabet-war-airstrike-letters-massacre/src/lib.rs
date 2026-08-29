//! <https://www.codewars.com/kata/5938f5b606c3033f4700015a/train/rust>

#![feature(portable_simd)]

use core::cmp::Ordering;
use std::simd::{Select, prelude::*};

const LANES: usize = 32;
const LETTERS: [u8; 8] = *b"wpbsmqdz";
const POWERS: [i8; 8] = [-4, -3, -2, -1, 4, 3, 2, 1];
const MAX_ABS_POWER: i8 = {
    let mut res = POWERS[0];
    let mut i = 0;
    while i < POWERS.len() {
        if POWERS[i] > res {
            res = POWERS[i];
        }
        i += 1;
    }
    res
};

pub fn alphabet_war(fight: &str) -> &'static str {
    match score_alphabet_war(fight.as_bytes()).cmp(&0) {
        Ordering::Less => "Left side wins!",
        Ordering::Equal => "Let's fight again!",
        Ordering::Greater => "Right side wins!",
    }
}

pub fn score_alphabet_war(fight: &[u8]) -> i64 {
    let n = fight.len();
    if n == 0 {
        return 0;
    }

    let star = Simd::<u8, LANES>::splat(b'*');
    let mut acc = Simd::<i8, LANES>::splat(0);
    let mut res = 0;
    let mut iters = 0u32;

    let mut p = 1;
    while p + LANES < n {
        let l = Simd::<u8, LANES>::from_slice(&fight[p - 1..p - 1 + LANES]);
        let m = Simd::<u8, LANES>::from_slice(&fight[p..p + LANES]);
        let r = Simd::<u8, LANES>::from_slice(&fight[p + 1..p + 1 + LANES]);

        let power = lookup_power(m);
        acc += (l.simd_eq(star) | r.simd_eq(star)).select(Simd::splat(0i8), power);

        iters += 1;
        if iters == (i8::MAX / MAX_ABS_POWER) as u32 {
            res += acc.cast::<i16>().reduce_sum() as i64;
            acc = Simd::splat(0);
            iters = 0;
        }
        p += LANES;
    }
    res += acc.cast::<i16>().reduce_sum() as i64;

    res += unsafe { contrib(fight, 0, n) };
    for i in p..n {
        res += unsafe { contrib(fight, i, n) };
    }
    res
}

#[cfg(target_arch = "x86_64")]
const fn build_tables() -> ([i8; 16], [i8; 16]) {
    let (mut t6, mut t7) = ([0i8; 16], [0i8; 16]);
    let mut k = 0;
    while k < 8 {
        let b = LETTERS[k];
        let lo = (b & 0x0f) as usize;
        match b & 0xf0 {
            0x60 => t6[lo] = POWERS[k],
            0x70 => t7[lo] = POWERS[k],
            _ => panic!(""),
        }
        k += 1;
    }
    (t6, t7)
}

#[cfg(target_arch = "x86_64")]
const T6: [i8; 16] = build_tables().0;

#[cfg(target_arch = "x86_64")]
const T7: [i8; 16] = build_tables().1;

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn shuffle_lookup(m: Simd<u8, LANES>) -> Simd<i8, LANES> {
    use core::arch::x86_64::{
        _mm_loadu_si128, _mm256_and_si256, _mm256_broadcastsi128_si256, _mm256_cmpeq_epi8,
        _mm256_or_si256, _mm256_set1_epi8, _mm256_shuffle_epi8,
    };

    let t6 = _mm256_broadcastsi128_si256(unsafe { _mm_loadu_si128(T6.as_ptr().cast()) });
    let t7 = _mm256_broadcastsi128_si256(unsafe { _mm_loadu_si128(T7.as_ptr().cast()) });
    let lo = _mm256_and_si256(m.into(), _mm256_set1_epi8(0x0f));
    let hi = _mm256_and_si256(m.into(), _mm256_set1_epi8(0xf0u8 as i8));
    _mm256_or_si256(
        _mm256_and_si256(
            _mm256_shuffle_epi8(t6, lo),
            _mm256_cmpeq_epi8(hi, _mm256_set1_epi8(0x60)),
        ),
        _mm256_and_si256(
            _mm256_shuffle_epi8(t7, lo),
            _mm256_cmpeq_epi8(hi, _mm256_set1_epi8(0x70)),
        ),
    )
    .into()
}

fn portable_lookup(m: Simd<u8, LANES>) -> Simd<i8, LANES> {
    let mut power = Simd::<i8, LANES>::splat(0);
    for k in 0..LETTERS.len() {
        power = m
            .simd_eq(Simd::splat(LETTERS[k]))
            .select(Simd::splat(POWERS[k]), power);
    }
    power
}

fn lookup_power(m: Simd<u8, LANES>) -> Simd<i8, LANES> {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        return unsafe { shuffle_lookup(m) };
    }
    portable_lookup(m)
}

unsafe fn contrib(fight: &[u8], i: usize, n: usize) -> i64 {
    let left = i > 0 && fight[i - 1] == b'*';
    let right = i + 1 < n && fight[i + 1] == b'*';
    if left || right {
        return 0;
    }

    let c = *unsafe { fight.get_unchecked(i) };
    for k in 0..8 {
        if LETTERS[k] == c {
            return POWERS[k] as i64;
        }
    }
    0
}
