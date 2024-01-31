#![feature(test)]

extern crate test;
use race_ceremony::race_podium;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(race_podium(black_box(100_000)));
        }
    });
}
