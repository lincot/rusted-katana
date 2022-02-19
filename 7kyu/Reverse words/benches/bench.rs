#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let words = black_box("Число сочетаний из n по k равно биномиальному коэффициенту");
    bencher.iter(|| solution::reverse_words(words))
}
