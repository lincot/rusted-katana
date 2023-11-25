#![no_std]
#![feature(test)]

extern crate test;
use from_a_to_z::gimme_the_letters;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(gimme_the_letters(black_box("e-k")));
            black_box(gimme_the_letters(black_box("B-Y")));
        }
    });
}
