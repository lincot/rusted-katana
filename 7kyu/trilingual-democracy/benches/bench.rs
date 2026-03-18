#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use trilingual_democracy::trilingual_democracy;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| trilingual_democracy(black_box(b"DFK")));
}
