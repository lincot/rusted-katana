#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let trips = black_box(&[(218, 224), (69, 84), (193, 204), (323, 333), (312, 338)]);
    bencher.iter(|| solution::days_represented(trips))
}
