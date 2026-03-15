#![feature(test)]

extern crate test;
use is_he_gonna_survive::hero;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| hero(black_box(40), black_box(10)));
}
