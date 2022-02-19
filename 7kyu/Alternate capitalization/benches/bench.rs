#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("абракадабра");
    bencher.iter(|| solution::capitalize(s))
}
