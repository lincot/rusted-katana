#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const TRIPS: [(u32, u32); 5] = [(218, 224), (69, 84), (193, 204), (323, 333), (312, 338)];

#[bench]
fn bench(bencher: &mut Bencher) {
    let trips = black_box(&TRIPS);

    bencher.iter(|| solution::days_represented(trips))
}
