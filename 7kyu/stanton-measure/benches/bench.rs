#![feature(test)]

extern crate test;
use stanton_measure::stanton_measure;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| stanton_measure(black_box(&[1, 4, 3, 2, 1, 2, 3, 2])));
}
