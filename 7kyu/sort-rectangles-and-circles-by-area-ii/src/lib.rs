//! <https://www.codewars.com/kata/5a1ebc2480171f29cf0000e5/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::{
    cmp::Ordering,
    f64::consts::PI,
    hint::unreachable_unchecked,
    mem::{forget, size_of},
};
use either::Either;

const fn gcd(mut m: usize, mut n: usize) -> usize {
    if m == 0 || n == 0 {
        return m | n;
    }
    let shift = (m | n).trailing_zeros();
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();
    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

pub fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    const N: usize = size_of::<Either<(f64, f64), f64>>()
        / gcd(
            size_of::<Either<(f64, f64), f64>>(),
            size_of::<(Either<(f64, f64), f64>, f64)>(),
        ); // 3 for 64 bit
    let mut seq_with_areas = Vec::with_capacity(
        seq.len()
            + if seq.len() % N == 0 {
                0
            } else {
                N - seq.len() % N
            },
    );
    unsafe { seq_with_areas.set_len(seq.len()) };
    for (a, &rectangle_or_circle) in seq_with_areas.iter_mut().zip(seq) {
        let area = rectangle_or_circle.either(|(a, b)| a * b, |r| PI * r * r);
        assert!(!area.is_nan());
        *a = (rectangle_or_circle, area);
    }
    if seq_with_areas.len() < 10000 {
        seq_with_areas.sort_unstable_by(|a, b| {
            if a.1.is_nan() || b.1.is_nan() {
                unsafe { unreachable_unchecked() }
            } else if a.1 > b.1 {
                Ordering::Greater
            } else if a.1 < b.1 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    } else {
        radsort::sort_by_key(&mut seq_with_areas, |x| x.1);
    }
    let mut res_ptr = seq_with_areas.as_mut_ptr().cast();
    for a in &seq_with_areas {
        unsafe {
            *res_ptr = a.0;
            res_ptr = res_ptr.add(1);
        }
    }
    let res = unsafe {
        Vec::from_raw_parts(
            seq_with_areas.as_mut_ptr().cast(),
            seq_with_areas.len(),
            seq_with_areas.capacity() * size_of::<(Either<(f64, f64), f64>, f64)>()
                / size_of::<Either<(f64, f64), f64>>(),
        )
    };
    forget(seq_with_areas);
    res
}
