#![feature(test)]

extern crate test;
use reduce_my_fraction::reduce_fraction;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| reduce_fraction(black_box((10_956_590, 13_611_876))));
}
