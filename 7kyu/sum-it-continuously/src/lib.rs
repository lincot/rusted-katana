//! <https://www.codewars.com/kata/59b44d00bf10a439dd00006f/train/rust>

#![feature(portable_simd)]

use core::slice;
use std::simd::{Select, prelude::*, simd_swizzle};

const LANES: usize = 8;
type V = Simd<i32, LANES>;

/// (a b c d | e f g h) -> (0 a b c | 0 e f g)
fn shl1_halves(v: V) -> V {
    simd_swizzle!(v, V::from_array([0; LANES]), [8, 0, 1, 2, 8, 4, 5, 6])
}

/// (a b c d | e f g h) -> (0 0 a b | 0 0 e f)
fn shl2_halves(v: V) -> V {
    simd_swizzle!(v, V::from_array([0; LANES]), [8, 8, 0, 1, 8, 8, 4, 5])
}

/// (a b c d | e f g h) -> (0 0 0 0 | d d d d)
fn low_last_to_high(v: V) -> V {
    simd_swizzle!(v, V::from_array([0; LANES]), [8, 8, 8, 8, 3, 3, 3, 3])
}

fn scan8(v: V) -> V {
    let v = v + shl1_halves(v); // a, a+b, b+c, c+d       (per half)
    let v = v + shl2_halves(v); // a, a+b, a+b+c, a+b+c+d (per half)
    v + low_last_to_high(v) // combine
}

/// Same result as [`scan8`], but faster.
///
/// # SAFETY
///
/// Data in `p[-1..8]` must be initialized.
unsafe fn scan8_overlapping(p: *const i32) -> V {
    let (v, v_left) = unsafe {
        (
            V::from_slice(slice::from_raw_parts(p, LANES)),
            V::from_slice(slice::from_raw_parts(p.sub(1), LANES)),
        )
    };

    // a, a+b, b+c, c+d (per half)
    let v = Mask::<i32, LANES>::from_bitmask(0b0001_0001).select(v, v + v_left);
    let v = v + shl2_halves(v); // a, a+b, a+b+c, a+b+c+d (per half)
    v + low_last_to_high(v) // combine
}

fn splat_last(v: V) -> V {
    simd_swizzle!(v, [LANES - 1; LANES])
}

#[target_feature(enable = "avx2")]
fn add_avx2(list: &[i32]) -> Vec<i32> {
    let len = list.len();
    let mut res: Vec<i32> = Vec::with_capacity(len);
    let dst = res.as_mut_ptr();
    let src = list.as_ptr();
    let mut carry = V::splat(0);
    let mut i = 0;

    if len >= LANES {
        unsafe {
            let v = scan8(V::from_slice(list.get_unchecked(0..LANES)));
            v.copy_to_slice(slice::from_raw_parts_mut(dst, LANES));
            carry = splat_last(v);
            i = LANES;

            while i + LANES <= len {
                let sums = scan8_overlapping(src.add(i));
                (sums + carry).copy_to_slice(slice::from_raw_parts_mut(dst.add(i), LANES));
                carry += splat_last(sums);
                i += LANES;
            }
        }
    }

    unsafe {
        let mut s = carry.as_array()[0];
        while i < len {
            s += *list.get_unchecked(i);
            *dst.add(i) = s;
            i += 1;
        }
        res.set_len(len);
    }
    res
}

fn scalar(list: &[i32]) -> Vec<i32> {
    let mut sum = 0;
    list.iter()
        .map(|x| {
            sum += x;
            sum
        })
        .collect()
}

pub fn add(list: &[i32]) -> Vec<i32> {
    if is_x86_feature_detected!("avx2") {
        unsafe { add_avx2(list) }
    } else {
        scalar(list)
    }
}
