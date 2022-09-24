#![no_std]
#![feature(test)]

extern crate test;
use how_many_days_are_we_represented_in_a_foreign_country::days_represented;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let trips = black_box(&[(218, 224), (69, 84), (193, 204), (323, 333), (312, 338)]);
    bencher.iter(|| days_represented(trips));
}
