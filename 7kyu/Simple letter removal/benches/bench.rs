#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("abracadabra");
    let k = black_box(6);
    bencher.iter(|| solution::solve(s, k))
}
