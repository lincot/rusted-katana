#![feature(test)]

extern crate test;
use pell_numbers::pell;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pell(black_box(100)));
}
