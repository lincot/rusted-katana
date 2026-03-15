#![feature(test)]

extern crate test;
use power_of_two::power_of_two;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| power_of_two(black_box(1000)));
}
