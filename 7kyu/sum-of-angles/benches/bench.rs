#![feature(test)]

extern crate test;
use sum_of_angles::angle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| angle(black_box(5)));
}
