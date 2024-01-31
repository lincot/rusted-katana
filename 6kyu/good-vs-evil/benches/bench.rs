#![feature(test)]

extern crate test;
use good_vs_evil::good_vs_evil;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        good_vs_evil(
            black_box("43445 7867 138 0 4562 10"),
            black_box("4358 8719 786 123 7863 783 1"),
        )
    });
}
