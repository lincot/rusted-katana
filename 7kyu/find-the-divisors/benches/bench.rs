#![feature(test)]

extern crate test;
use find_the_divisors::divisors;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| divisors(black_box(1_441_440)));
}
