#![no_std]
#![feature(test)]

extern crate test;
use growth_of_a_population::nb_year;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(nb_year(
                black_box(1_500_000),
                black_box(2.5),
                black_box(10_000),
                black_box(2_000_000),
            ));
        }
    });
}
