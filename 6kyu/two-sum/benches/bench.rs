#![feature(test)]

extern crate test;
use core::array;

use test::{Bencher, black_box};
use two_sum::two_sum;

#[bench]
fn bench(bencher: &mut Bencher) {
    let numbers: [_; if cfg!(miri) { 10 } else { 1000 }] = array::from_fn(|i| i as i32 * i as i32);
    bencher.iter(|| {
        two_sum(
            black_box(&numbers),
            black_box(if cfg!(miri) {
                2 * 2 + 6 * 6
            } else {
                113 * 113 + 800 * 800
            }),
        )
    });
}
