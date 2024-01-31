//! <https://www.codewars.com/kata/5a1ebc2480171f29cf0000e5/train/rust>

use core::{cmp::Ordering, f64::consts::PI};
use either::Either;

#[derive(PartialEq)]
struct OrdF64(f64);

impl Eq for OrdF64 {}

impl PartialOrd for OrdF64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdF64 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

fn calc_area(rectangle_or_circle: &Either<(f64, f64), f64>) -> f64 {
    rectangle_or_circle.either(|(a, b)| a * b, |r| PI * r * r)
}

pub fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let mut res = seq.to_vec();
    if res.len() < 6666 {
        res.sort_by_cached_key(|x| OrdF64(calc_area(x)));
    } else {
        radsort::sort_by_cached_key(&mut res, calc_area);
    }
    res
}
