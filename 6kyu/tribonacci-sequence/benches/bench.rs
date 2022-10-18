#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use tribonacci_sequence::tribonacci;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| tribonacci(black_box(&[0.5, 0.5, 0.5]), black_box(30)));
}
