#![feature(test)]

extern crate test;
use min_factor_distance::min_distance;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for n in 1000..if cfg!(miri) { 1010 } else { 1100 } {
            black_box(min_distance(black_box(n)));
        }
    });
}
