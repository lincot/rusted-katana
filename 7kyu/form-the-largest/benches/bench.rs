#![no_std]
#![feature(test)]

extern crate test;
use form_the_largest::max_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(max_number(black_box(467_678)));
        }
    });
}
