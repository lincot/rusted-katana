#![feature(test)]

extern crate test;
use does_my_number_look_big_in_this::narcissistic;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| narcissistic(black_box(4887)));
}
