#![feature(test)]

extern crate test;
use how_much::how_much;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| how_much(black_box(10000), black_box(9900)));
}
