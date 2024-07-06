#![feature(test)]

extern crate test;
use sixteen_circles::sixteen_circles;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sixteen_circles(black_box(283)));
}
