#![feature(test)]

extern crate test;
use chasers_schedule::run;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(run(black_box(829), black_box(135)));
        black_box(run(black_box(49), black_box(50)));
    });
}
