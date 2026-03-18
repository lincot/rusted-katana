#![feature(test)]

extern crate test;
use maximum_multiple::max_multiple;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| max_multiple(black_box(37), black_box(200)));
}
