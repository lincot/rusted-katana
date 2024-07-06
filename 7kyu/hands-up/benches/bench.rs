#![feature(test)]

extern crate test;
use hands_up::get_positions;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_positions(black_box(1337)));
}
