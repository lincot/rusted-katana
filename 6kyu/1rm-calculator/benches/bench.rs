#![feature(test)]

extern crate test;
use solution_1rm_calculator::calculate_1_rm;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(calculate_1_rm(black_box(135), black_box(4)));
        black_box(calculate_1_rm(black_box(135), black_box(7)));
        black_box(calculate_1_rm(black_box(135), black_box(20)));
    });
}
