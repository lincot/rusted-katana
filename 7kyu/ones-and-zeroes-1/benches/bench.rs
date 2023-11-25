#![no_std]
#![feature(test)]

extern crate test;
use ones_and_zeroes_1::same_length;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(same_length(black_box("1100111000100")));
            black_box(same_length(black_box("1011100010")));
            black_box(same_length(black_box("101111000011001011110000101100")));
            black_box(same_length(black_box(
                "10111100001111001110110010111100001111",
            )));
        }
    });
}
