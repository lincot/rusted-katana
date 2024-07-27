#![feature(test)]

extern crate test;
use build_tower_advanced::tower_builder;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| tower_builder(black_box(30), black_box((3, 3))));
}
