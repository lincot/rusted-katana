#![feature(test)]

extern crate test;
use count_the_divisors_of_a_number::divisors;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| divisors(black_box(3_491_888_400)));
}
