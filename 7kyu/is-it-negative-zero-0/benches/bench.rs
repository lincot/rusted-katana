#![feature(test)]

extern crate test;
use is_it_negative_zero_0::is_negative_zero;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| is_negative_zero(black_box(-0.)));
}
