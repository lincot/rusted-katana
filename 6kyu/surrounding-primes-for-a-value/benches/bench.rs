#![feature(test)]

extern crate test;
use surrounding_primes_for_a_value::prime_bef_aft;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| prime_bef_aft(black_box(200_000)));
}
