#![no_std]
#![feature(test)]

extern crate test;
use evens_and_odds::evens_and_odds;
use test::{black_box, Bencher};

#[bench]
fn bench_15800(bencher: &mut Bencher) {
    let n = black_box(15800);
    bencher.iter(|| evens_and_odds(n));
}

#[bench]
fn bench_15801(bencher: &mut Bencher) {
    let n = black_box(15801);
    bencher.iter(|| evens_and_odds(n));
}
