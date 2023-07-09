#![no_std]
#![feature(test)]

extern crate test;
use element_equals_its_index::index_equals_value;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(index_equals_value(black_box(&[
                -5, 1, 2, 3, 4, 5, 7, 10, 15,
            ])));
        }
    });
}
