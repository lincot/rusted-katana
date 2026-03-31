#![feature(test)]

extern crate test;
use core::ops::Sub;

use test::{Bencher, black_box};
use zipwith::zip_with;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        zip_with(
            black_box(i32::sub),
            black_box(&[0, 1, 2, 3, 4, 5]),
            black_box(&[6, 5, 4, 3, 2, 1]),
        )
    });
}
