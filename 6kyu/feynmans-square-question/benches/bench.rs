#![feature(test)]

extern crate test;
use feynmans_square_question::count_squares;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_squares(black_box(100)));
}
