#![feature(test)]

extern crate test;
use backwards_read_primes::backwards_prime;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        backwards_prime(
            black_box(if cfg!(miri) { 1 } else { 100_000 }),
            black_box(if cfg!(miri) { 1000 } else { 110_000 }),
        )
    });
}
