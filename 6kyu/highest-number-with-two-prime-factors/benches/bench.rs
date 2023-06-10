#![no_std]
#![feature(test)]

extern crate test;
use highest_number_with_two_prime_factors::highest_bi_prime_factor;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        highest_bi_prime_factor(
            black_box(211),
            black_box(367),
            black_box(464_350_116_292_453_u64.into()),
        );
    });
}
