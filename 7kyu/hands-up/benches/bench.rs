#![no_std]
#![feature(test)]

extern crate test;
use hands_up::get_positions;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            get_positions(black_box(1337));
        }
    });
}
