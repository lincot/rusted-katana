#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use thinking_and_testing_how_many_word::testit;

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(
        "When you in order to do something by a wrong way, your heart will missed somewhere.",
    );
    bencher.iter(|| testit(s));
}
