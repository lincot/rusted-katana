#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let row_str = black_box("RBRGBRBGGRRRBGBBBGG");
    bencher.iter(|| solution::triangle(row_str))
}
