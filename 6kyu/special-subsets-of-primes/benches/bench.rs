#![feature(test)]

extern crate test;
use special_subsets_of_primes::special_primes;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| special_primes(black_box(40_000)));
}
