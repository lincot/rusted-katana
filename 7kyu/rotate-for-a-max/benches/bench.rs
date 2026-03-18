#![feature(test)]

extern crate test;
use rotate_for_a_max::max_rot;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| max_rot(black_box(195_881_031)));
}
