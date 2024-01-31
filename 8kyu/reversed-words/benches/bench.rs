#![feature(test)]

extern crate test;
use reversed_words::reverse_words;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        reverse_words(black_box(
            "Число сочетаний из n по k равно биномиальному коэффициенту",
        ))
    });
}
