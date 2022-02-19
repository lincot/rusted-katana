#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let sea = black_box("_~~~~~~~_~__~______~~__~~_~~");
    bencher.iter(|| solution::sea_sick(sea))
}
