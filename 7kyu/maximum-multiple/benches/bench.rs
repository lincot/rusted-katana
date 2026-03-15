#![feature(test)]

extern crate test;
use maximum_multiple::max_multiple;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| max_multiple(black_box(37), black_box(200)));
}
