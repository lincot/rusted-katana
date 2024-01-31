#![feature(test)]

extern crate test;
use sum_two_arrays::add_arrays;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(add_arrays(
                black_box(&[3, 2, 6, 6, 1]),
                black_box(&[-7, 2, 2, 8, 5]),
            ));
        }
    });
}
