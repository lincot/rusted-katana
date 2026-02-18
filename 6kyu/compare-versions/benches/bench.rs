#![feature(test)]

extern crate test;
use compare_versions::compare_versions;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| compare_versions(black_box("4.3.3"), black_box("4.3.3.1")));
}
