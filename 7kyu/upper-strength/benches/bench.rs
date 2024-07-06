#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use upper_strength::alex_mistakes;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| alex_mistakes(black_box(9), black_box(180)));
}
