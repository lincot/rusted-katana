#![feature(test)]

extern crate test;
use bugs_life::shortest_distance;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| shortest_distance(black_box(134.), black_box(191.5), black_box(45.5)));
}
