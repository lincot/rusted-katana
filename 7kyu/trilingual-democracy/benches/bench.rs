#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use trilingual_democracy::trilingual_democracy;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| trilingual_democracy(black_box(b"DFK")));
}
