#![feature(test)]

extern crate test;
use find_multiples_of_a_number::find_multiples;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(11);
    let limit = black_box(1000);
    bencher.iter(|| find_multiples(n, limit));
}
