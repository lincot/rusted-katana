#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use welcome::greet;

#[bench]
fn bench_estonian(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("estonian")));
}

#[bench]
fn bench_swedish(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("swedish")));
}
