#![feature(test)]

extern crate test;
use for_whom_the_bell_tolls::bell;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| bell(black_box(1000)));
}
