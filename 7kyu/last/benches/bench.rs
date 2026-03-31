#![feature(test)]

extern crate test;
use core::array;

use last::last;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let slice: [_; 16] = array::from_fn(|i| i);
    bencher.iter(|| last(black_box(&slice)));
}
