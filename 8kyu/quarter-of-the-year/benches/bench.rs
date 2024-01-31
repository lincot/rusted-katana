#![feature(test)]

extern crate test;
use quarter_of_the_year::quarter_of;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(quarter_of(black_box(3)));
        }
    });
}
