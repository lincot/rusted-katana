#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let good = black_box("43445 7867 138 0 4562 10");
    let evil = black_box("4358 8719 786 123 7863 783 1");
    bencher.iter(|| solution::good_vs_evil(good, evil));
}
