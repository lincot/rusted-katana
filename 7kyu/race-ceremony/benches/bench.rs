#![feature(test)]

extern crate test;
use race_ceremony::race_podium;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| race_podium(black_box(100_000)));
}
