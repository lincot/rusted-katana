#![feature(test)]

extern crate test;
use how_many_nines::nines;
use num_bigint::BigInt;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| nines(black_box(BigInt::from(1u8) << 100u8)));
}
