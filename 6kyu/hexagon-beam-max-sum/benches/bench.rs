#![feature(test)]

extern crate test;
use hexagon_beam_max_sum::max_hexagon_beam;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| max_hexagon_beam(black_box(5), black_box(&[1, 0, 4, -6])));
}
