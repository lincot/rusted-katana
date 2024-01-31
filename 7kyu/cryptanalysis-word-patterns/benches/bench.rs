#![feature(test)]

extern crate test;
use cryptanalysis_word_patterns::word_pattern;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| word_pattern(black_box("Hippopotomonstrosesquippedaliophobia")));
}
