#![feature(test)]

extern crate test;
use sum_of_two_integers::add;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| add(black_box(1), black_box(2)));
}
