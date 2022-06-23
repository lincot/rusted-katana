#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use transform_to_prime::minimum_number;

#[bench]
fn bench(bencher: &mut Bencher) {
    let xs = black_box(&[50, 39, 49, 6, 17, 28]);
    bencher.iter(|| minimum_number(xs));
}
