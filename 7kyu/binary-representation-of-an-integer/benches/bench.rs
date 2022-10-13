#![no_std]
#![feature(test)]

extern crate test;
use binary_representation_of_an_integer::show_bits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(-12336);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(show_bits(n));
        }
    });
}
