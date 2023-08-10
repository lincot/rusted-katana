#![no_std]
#![feature(test)]

extern crate test;
use color_choice::check_choose;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(check_choose(black_box(184_756), black_box(20)));
        }
    });
}
