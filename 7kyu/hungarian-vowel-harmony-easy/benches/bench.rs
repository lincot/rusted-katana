#![no_std]
#![feature(test)]

extern crate test;
use hungarian_vowel_harmony_easy::dative;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(dative(black_box("király")));
        }
    });
}
