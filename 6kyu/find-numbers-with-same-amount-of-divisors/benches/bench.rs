#![feature(test)]

extern crate test;
use find_numbers_with_same_amount_of_divisors::count_pairs_int;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_pairs_int(
            black_box(6),
            black_box(if cfg!(miri) { 100 } else { 10_000 }),
        )
    });
}
