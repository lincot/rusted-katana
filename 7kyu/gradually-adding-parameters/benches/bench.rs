#![no_std]
#![feature(test)]

extern crate test;
use gradually_adding_parameters::add;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(add(black_box(&[
                18, 36, 20, 29, 72, 69, 10, 82, 16, 30, 40, 54, 22, 68, 89, 60, 49, 67, 96, 28,
            ])));
        }
    });
}
