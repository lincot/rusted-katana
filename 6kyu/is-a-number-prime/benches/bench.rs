#![feature(test)]

extern crate test;
use is_a_number_prime::is_prime;
use test::{black_box, Bencher};

#[bench]
fn bench_big(bencher: &mut Bencher) {
    bencher.iter(|| is_prime(black_box(6_862_564_799_914_328_471)));
}

#[bench]
fn bench_small(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(is_prime(black_box(-41)));
        black_box(is_prime(black_box(41)));
        black_box(is_prime(black_box(73)));
        black_box(is_prime(black_box(75)));
        black_box(is_prime(black_box(5099)));
    });
}
