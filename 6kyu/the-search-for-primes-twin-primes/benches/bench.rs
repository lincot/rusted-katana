#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_search_for_primes_twin_primes::twin_prime;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| twin_prime(black_box(10_000)));
}
