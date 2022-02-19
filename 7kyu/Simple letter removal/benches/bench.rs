#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "abracadabra";
const K: usize = 6;

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(S);
    let k = black_box(K);

    bencher.iter(|| solution::solve(s, k))
}
