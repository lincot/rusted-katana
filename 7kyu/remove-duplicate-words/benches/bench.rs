#![feature(test)]

extern crate test;
use remove_duplicate_words::remove_duplicate_words;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(
        "alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta",
    );
    bencher.iter(|| remove_duplicate_words(s));
}
