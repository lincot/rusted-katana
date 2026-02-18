#![feature(test)]

extern crate test;
use pell_numbers::pell;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pell(black_box(100)));
}
