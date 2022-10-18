#![no_std]
#![feature(test)]

extern crate test;
use evens_and_odds::evens_and_odds;
use test::{black_box, Bencher};

#[bench]
fn bench_15800(bencher: &mut Bencher) {
    bencher.iter(|| evens_and_odds(black_box(15800)));
}

#[bench]
fn bench_15801(bencher: &mut Bencher) {
    bencher.iter(|| evens_and_odds(black_box(15801)));
}
