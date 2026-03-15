#![feature(test)]

extern crate test;
use keep_hydrated_1::litres;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for time in [2., 1.4, 12.3, 0.82, 11.8] {
            black_box(litres(black_box(time)));
        }
    });
}
