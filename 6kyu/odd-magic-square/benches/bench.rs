#![feature(test)]

extern crate test;
use odd_magic_square::magic_square;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| magic_square(black_box(15)));
}
