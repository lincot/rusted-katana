#![feature(test)]

extern crate test;
use sum_of_cubes::sum_cubes;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sum_cubes(black_box(123)));
}
