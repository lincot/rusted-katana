#![feature(test)]

extern crate test;
use hungarian_vowel_harmony_easy::dative;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| dative(black_box("király")));
}
