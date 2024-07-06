#![feature(test)]

extern crate test;
use financing_a_purchase::amort;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        amort(
            black_box(7.4),
            black_box(10_215),
            black_box(24),
            black_box(20),
        )
    });
}
