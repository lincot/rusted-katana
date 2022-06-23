#![feature(test)]

extern crate test;
use coloured_triangles::triangle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let row_str = black_box("RBRGBRBGGRRRBGBBBGG");
    bencher.iter(|| triangle(row_str));
}
