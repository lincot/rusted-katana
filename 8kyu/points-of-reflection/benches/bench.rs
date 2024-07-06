#![feature(test)]

extern crate test;
use points_of_reflection::symmetric_point;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| symmetric_point(black_box([1000, 15]), black_box([-7, -214])));
}
