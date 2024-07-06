#![feature(test)]

extern crate test;
use decompose_a_number::decompose;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| decompose(black_box(8_331_299)));
}
