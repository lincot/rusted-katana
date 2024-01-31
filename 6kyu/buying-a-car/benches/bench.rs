#![feature(test)]

extern crate test;
use buying_a_car::nb_months;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        nb_months(
            black_box(7500),
            black_box(32_000),
            black_box(300),
            black_box(1.55),
        )
    });
}
