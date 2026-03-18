#![feature(test)]

extern crate test;
use next_prime::next_prime;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| next_prime(black_box(78_152_045)));
}
