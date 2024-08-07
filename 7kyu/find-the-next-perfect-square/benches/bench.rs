#![feature(test)]

extern crate test;
use find_the_next_perfect_square::find_next_square;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_next_square(black_box(319_225)));
}
