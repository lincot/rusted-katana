#![feature(test)]

extern crate test;
use count_a_spiral::spiral_sum;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| spiral_sum(black_box(1000u16.into())));
}
