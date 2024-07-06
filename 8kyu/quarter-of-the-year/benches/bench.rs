#![feature(test)]

extern crate test;
use quarter_of_the_year::quarter_of;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| quarter_of(black_box(3)));
}
