#![feature(test)]

extern crate test;
use number_of_divisions::divisions;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| divisions(black_box(2450), black_box(5)));
}
