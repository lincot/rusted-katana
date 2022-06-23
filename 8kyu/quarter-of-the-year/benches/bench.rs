#![feature(test)]

extern crate test;
use quarter_of_the_year::quarter_of;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let month = black_box(3);
    bencher.iter(|| {
        for _ in 0..1_000_000 {
            black_box(quarter_of(month));
        }
    });
}
