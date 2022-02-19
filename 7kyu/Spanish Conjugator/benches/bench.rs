#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let verb = black_box("vivir");
    bencher.iter(|| solution::conjugate(verb))
}
