#![feature(test)]

extern crate test;
use collatz_conjecture_length::collatz;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| collatz(black_box(500)));
}
