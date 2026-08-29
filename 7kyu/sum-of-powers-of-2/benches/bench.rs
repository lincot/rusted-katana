#![feature(test)]

extern crate test;
use sum_of_powers_of_2::powers;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for i in 0..1000 {
            black_box(powers(black_box(i)));
            black_box(powers(black_box(u32::MAX / 2 - i)));
        }
    });
}
