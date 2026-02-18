#![feature(test)]

extern crate test;
use duplicate_encoder::duplicate_encode;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| duplicate_encode(black_box("Suksess")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| duplicate_encode(black_box("Саксесс")));
}
