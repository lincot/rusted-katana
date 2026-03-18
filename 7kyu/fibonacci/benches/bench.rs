#![feature(test)]

extern crate test;
use fibonacci::fib;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| fib(black_box(30)));
}
