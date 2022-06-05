#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(11);
    let limit = black_box(1000);
    bencher.iter(|| solution::find_multiples(n, limit));
}
