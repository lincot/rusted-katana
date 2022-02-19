#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const ROW_STR: &str = "RBRGBRBGGRRRBGBBBGG";

#[bench]
fn bench(bencher: &mut Bencher) {
    let row_str = black_box(ROW_STR);

    bencher.iter(|| solution::triangle(row_str))
}
