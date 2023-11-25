#![no_std]
#![feature(test)]

extern crate test;
use reverse_a_number_in_any_base::reversed_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            reversed_number(black_box(3_040_620_649), black_box(7));
        }
    });
}
