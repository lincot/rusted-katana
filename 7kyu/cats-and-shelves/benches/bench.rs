#![feature(test)]

extern crate test;
use cats_and_shelves::cats_and_shelves;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| cats_and_shelves(black_box(2), black_box(5)));
}
