#![feature(test)]

extern crate test;
use alternate_capitalization::capitalize;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| capitalize(black_box("abracadabra")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| capitalize(black_box("абракадабра")));
}
