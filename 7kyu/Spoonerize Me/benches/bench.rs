#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const WORDS: &str = "Дионисий Младший";

#[bench]
fn bench(bencher: &mut Bencher) {
    let words = black_box(WORDS);

    bencher.iter(|| solution::spoonerize(words))
}
