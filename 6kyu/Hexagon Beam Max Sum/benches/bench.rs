#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(5);
    let seq = black_box(&[1, 0, 4, -6]);
    bencher.iter(|| solution::max_hexagon_beam(n, seq))
}
