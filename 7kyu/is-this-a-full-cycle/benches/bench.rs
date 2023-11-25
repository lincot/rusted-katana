#![no_std]
#![feature(test)]

extern crate test;
use is_this_a_full_cycle::full_cycle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        full_cycle(black_box(&[
            2, 17, 7, 19, 18, 9, 5, 15, 16, 8, 11, 6, 14, 4, 3, 13, 0, 12, 1, 10,
        ]))
    });
}
