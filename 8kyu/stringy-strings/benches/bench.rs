#![feature(test)]

extern crate test;
use stringy_strings::stringy;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| stringy(black_box(if cfg!(miri) { 61 } else { 1001 })));
}
