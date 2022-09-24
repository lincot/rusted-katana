#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use tribonacci_sequence::tribonacci;

#[bench]
fn bench(bencher: &mut Bencher) {
    let signature = black_box(&[0.5, 0.5, 0.5]);
    let n = black_box(30);
    bencher.iter(|| tribonacci(signature, n));
}
