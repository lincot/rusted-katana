#![feature(test)]

extern crate test;
use going_to_the_cinema::movie;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| movie(black_box(500), black_box(15), black_box(0.9)));
}
