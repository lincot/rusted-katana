#![feature(test)]

extern crate test;
use hands_up::get_positions;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_positions(black_box(1337)));
}
