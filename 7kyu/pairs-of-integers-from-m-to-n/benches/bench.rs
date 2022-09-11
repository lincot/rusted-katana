#![feature(test)]

extern crate test;
use pairs_of_integers_from_m_to_n::generate_pairs;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let m = black_box(2);
    let n = black_box(100);
    bencher.iter(|| generate_pairs(m, n));
}
