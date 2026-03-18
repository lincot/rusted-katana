#![feature(test)]

extern crate test;
use latin_squares::make_latin_square;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| make_latin_square(black_box(10)));
}
