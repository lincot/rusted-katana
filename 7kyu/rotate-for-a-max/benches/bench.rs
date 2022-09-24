#![no_std]
#![feature(test)]

extern crate test;
use rotate_for_a_max::max_rot;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box(195_881_031);
    bencher.iter(|| max_rot(num));
}
