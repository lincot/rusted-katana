#![feature(test)]

extern crate test;
use rotate_for_a_max::max_rot;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            max_rot(black_box(195_881_031));
        }
    });
}
