#![feature(test)]

extern crate test;
use integers_recreation_two::prod2sum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(prod2sum(
                black_box(136),
                black_box(35),
                black_box(116),
                black_box(375),
            ));
        }
    });
}
