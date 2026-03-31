#![feature(test)]

extern crate test;
use core::array;

use calculate_average::find_average;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let slice: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|i| i as f64);
    bencher.iter(|| find_average(black_box(&slice)));
}
