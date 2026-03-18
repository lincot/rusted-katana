#![feature(test)]

extern crate test;
use coloured_triangles::triangle;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| triangle(black_box("RBRGBRBGGRRRBGBBBGG")));
}
