#![feature(test)]

extern crate test;
use collatz::collatz;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(collatz(black_box(1000)));
        black_box(collatz(black_box(1100)));
        black_box(collatz(black_box(1_000_000)));
    });
}
