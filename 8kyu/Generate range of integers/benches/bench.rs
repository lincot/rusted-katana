#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let min = black_box(1000);
    let max = black_box(2000);
    let step = black_box(9);
    bencher.iter(|| solution::generate_range(min, max, step));
}
