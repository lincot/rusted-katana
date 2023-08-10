#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use upper_strength::alex_mistakes;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(alex_mistakes(black_box(9), black_box(180)));
        }
    });
}
