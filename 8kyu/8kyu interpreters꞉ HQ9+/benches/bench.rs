#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let code = black_box("9");
    bencher.iter(|| solution::hq9(code))
}
