#![no_std]
#![feature(test)]

extern crate test;
use reverse_words::reverse_words;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let words = black_box("Число сочетаний из n по k равно биномиальному коэффициенту");
    bencher.iter(|| reverse_words(words));
}
