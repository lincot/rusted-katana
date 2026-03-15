#![feature(test)]

extern crate test;
use alphabetically_ordered::alphabetic;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| alphabetic(black_box("abcdefghijklmnop")));
}
