#![no_std]
#![feature(test)]

extern crate test;
use find_the_middle_element::gimme;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(gimme(black_box([1, 2, 3])));
            black_box(gimme(black_box([2, 3, 1])));
            black_box(gimme(black_box([3, 1, 2])));
        }
    });
}
