#![feature(test)]

extern crate test;
use persistent_bugger::persistence;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| persistence(black_box(1_254_859_723)));
}
