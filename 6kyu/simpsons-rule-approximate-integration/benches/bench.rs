#![feature(test)]

extern crate test;
use simpsons_rule_approximate_integration::simpson;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| simpson(black_box(100)));
}
