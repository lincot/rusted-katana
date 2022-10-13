#![no_std]
#![feature(test)]

extern crate test;
use next_prime::next_prime;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(78_152_045);
    bencher.iter(|| next_prime(n));
}
