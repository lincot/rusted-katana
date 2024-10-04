#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use tortoise_racing::race;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| race(black_box(80), black_box(91), black_box(37)));
}
