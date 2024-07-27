#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_sum_of_the_prime_factors_of_a_number_dot_dot_dot_what_for::mult_primefactor_sum;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        mult_primefactor_sum(
            black_box(50_000),
            black_box(if cfg!(miri) { 50_300 } else { 51_000 }),
        )
    });
}
