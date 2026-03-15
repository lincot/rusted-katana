#![feature(test)]

extern crate test;
use bin_to_decimal::bin_to_decimal;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| bin_to_decimal(black_box("100100101010110")));
}
