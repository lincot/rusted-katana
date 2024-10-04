#![feature(test)]

extern crate test;
use braking_well::{dist, speed};
use test::{black_box, Bencher};

#[bench]
fn bench_dist(bencher: &mut Bencher) {
    bencher.iter(|| dist(black_box(92.), black_box(0.5)));
}

#[bench]
fn bench_speed(bencher: &mut Bencher) {
    bencher.iter(|| speed(black_box(164.0), black_box(0.7)));
}
