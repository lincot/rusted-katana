#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let radius = black_box(200);
    bencher.iter(|| solution::circle(radius))
}
