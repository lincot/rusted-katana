#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use valid_braces::valid_braces;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| valid_braces(black_box("({})[({})]")));
}
