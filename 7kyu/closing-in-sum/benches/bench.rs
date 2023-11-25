#![no_std]
#![feature(test)]

extern crate test;
use closing_in_sum::closing_in_sum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(closing_in_sum(black_box(8_780_436_960_754_480_978)));
        }
    });
}
