#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use tribonacci_sequence::tribonacci;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| tribonacci(black_box(&[0.5, 0.5, 0.5]), black_box(30)));
}
