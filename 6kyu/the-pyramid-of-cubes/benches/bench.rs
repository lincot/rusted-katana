#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_pyramid_of_cubes::find_height;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_height(black_box(40_000)));
}
