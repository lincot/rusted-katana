#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const SEA: &str = "_~~~~~~~_~__~______~~__~~_~~";

#[bench]
fn bench(bencher: &mut Bencher) {
    let sea = black_box(SEA);

    bencher.iter(|| solution::sea_sick(sea))
}
