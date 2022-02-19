#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const WORDS: &str = "Число сочетаний из n по k равно биномиальному коэффициенту";

#[bench]
fn bench(bencher: &mut Bencher) {
    let words = black_box(WORDS);

    bencher.iter(|| solution::reverse_words(words))
}
