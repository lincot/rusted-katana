#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_sum_of_the_first_and_the_last_prime_factor_make_chains_of_numbers::sflpf_data;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        sflpf_data(
            black_box(501),
            black_box(if cfg!(miri) { 1000 } else { 5000 }),
        )
    });
}
