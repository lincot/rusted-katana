#![feature(test)]

extern crate test;
use core::array;

use find_the_missing_term_in_an_arithmetic_progression::find_missing;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const LEN: usize = if cfg!(miri) { 16 } else { 1024 };
    const STEP: i32 = 5;
    let seq: [_; LEN] = array::from_fn(|i| {
        if i < LEN / 2 {
            STEP * i as i32
        } else {
            STEP * (i + 1) as i32
        }
    });
    bencher.iter(|| find_missing(black_box(&seq)));
}
