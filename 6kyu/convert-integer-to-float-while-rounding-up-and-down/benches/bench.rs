#![feature(test)]

extern crate test;
use convert_integer_to_float_while_rounding_up_and_down::{iceil, ifloor};
use num_bigint::BigInt;
use test::{black_box, Bencher};

#[bench]
fn bench_ifloor(bencher: &mut Bencher) {
    let n = BigInt::from(2).pow(60) - 1;
    bencher.iter(|| ifloor(black_box(&n)));
}

#[bench]
fn bench_iceil(bencher: &mut Bencher) {
    let n = BigInt::from(2).pow(60) - 1;
    bencher.iter(|| iceil(black_box(&n)));
}
