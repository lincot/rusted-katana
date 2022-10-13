#![no_std]
#![feature(test)]

extern crate test;
use race_ceremony::race_podium;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let blocks = black_box(100_000);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(race_podium(blocks));
        }
    });
}
