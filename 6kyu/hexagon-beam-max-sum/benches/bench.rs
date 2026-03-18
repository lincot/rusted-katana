#![feature(test)]

extern crate test;
use hexagon_beam_max_sum::max_hexagon_beam;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        max_hexagon_beam(
            black_box(if cfg!(miri) { 2 } else { 92 }),
            black_box(&[324, -90, 28, -331, 24, 55, 94, -101]),
        )
    });
}
