#![feature(test)]

extern crate test;
use quadrants::quadrant;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for x in [-1, 1] {
            for y in [-1, 1] {
                black_box(quadrant(black_box(x), black_box(y)));
            }
        }
    });
}
