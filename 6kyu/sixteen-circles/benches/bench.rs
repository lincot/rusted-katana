#![feature(test)]

extern crate test;
use sixteen_circles::sixteen_circles;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sixteen_circles(black_box(283)));
}
