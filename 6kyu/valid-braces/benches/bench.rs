#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use valid_braces::valid_braces;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| valid_braces(black_box("({})[({})]")));
}
