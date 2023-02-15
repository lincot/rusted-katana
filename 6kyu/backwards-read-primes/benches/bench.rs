#![no_std]
#![feature(test)]

extern crate test;
use backwards_read_primes::backwards_prime;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| backwards_prime(black_box(1), black_box(10_000)));
}
