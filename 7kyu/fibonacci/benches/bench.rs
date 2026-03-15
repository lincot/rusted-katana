#![feature(test)]

extern crate test;
use fibonacci::fib;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| fib(black_box(30)));
}
