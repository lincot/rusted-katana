#![no_std]
#![feature(test)]

extern crate test;
use a_wolf_in_sheeps_clothing::warn_the_sheep;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(warn_the_sheep(black_box(&[
                "sheep", "sheep", "sheep", "wolf", "sheep", "sheep", "sheep",
            ])));
        }
    });
}
