#![no_std]
#![feature(test)]

extern crate test;
use find_nearest_square_number::nearest_sq;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(786_737);
    bencher.iter(|| {
        for _ in 0..1_000_000 {
            black_box(nearest_sq(n));
        }
    });
}
