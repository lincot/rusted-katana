#![feature(test)]

extern crate test;
use red_knight::red_knight;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| red_knight(black_box(1), black_box(6)));
}
