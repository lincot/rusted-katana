#![feature(test)]

extern crate test;
use bugs_life::shortest_distance;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(shortest_distance(
                black_box(134.),
                black_box(191.5),
                black_box(45.5),
            ));
        }
    });
}
