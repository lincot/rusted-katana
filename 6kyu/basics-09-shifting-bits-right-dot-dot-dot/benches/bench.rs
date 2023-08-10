#![no_std]
#![feature(test)]

extern crate test;
use basics_09_shifting_bits_right_dot_dot_dot::previous_power_of_2;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(previous_power_of_2(black_box(1000)));
            black_box(previous_power_of_2(black_box(-1000)));
        }
    });
}
