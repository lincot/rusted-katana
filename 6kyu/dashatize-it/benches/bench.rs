#![feature(test)]

extern crate test;
use dashatize_it::dashatize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| dashatize(black_box(-28369)));
}
