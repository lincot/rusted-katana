#![feature(test)]

extern crate test;
use hexagon_beam_max_sum::max_hexagon_beam;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(5);
    let seq = black_box(&[1, 0, 4, -6]);
    bencher.iter(|| max_hexagon_beam(n, seq));
}
