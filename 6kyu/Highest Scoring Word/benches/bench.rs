#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input = black_box("what time are we climbing up the volcano");
    bencher.iter(|| solution::high(input))
}
