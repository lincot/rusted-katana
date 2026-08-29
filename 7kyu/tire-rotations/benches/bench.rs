#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use tire_rotations::tire_rotations;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| tire_rotations(black_box("205/55R16"), black_box(110)));
}
