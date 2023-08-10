#![no_std]
#![feature(test)]

extern crate test;
use find_nearest_square_number::nearest_sq;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(nearest_sq(black_box(786_737)));
        }
    });
}
