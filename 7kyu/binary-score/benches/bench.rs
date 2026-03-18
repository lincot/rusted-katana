#![feature(test)]

extern crate test;
use binary_score::score;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| score(black_box(2_376_499_510)));
}
