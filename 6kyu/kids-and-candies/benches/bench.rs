#![feature(test)]

extern crate test;
use kids_and_candies::candies_to_buy;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| candies_to_buy(black_box(1000)));
}
