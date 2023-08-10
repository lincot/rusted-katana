#![no_std]
#![feature(test)]

extern crate test;
use dashatize_it::dashatize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(dashatize(black_box(-28369)));
        }
    });
}
