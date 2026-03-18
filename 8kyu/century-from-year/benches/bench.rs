#![feature(test)]

extern crate test;
use century_from_year::century;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| century(black_box(2025)));
}
