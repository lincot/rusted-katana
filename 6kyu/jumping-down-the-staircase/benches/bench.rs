#![feature(test)]

extern crate test;
use jumping_down_the_staircase::get_number_of_ways;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_number_of_ways(black_box(96), black_box(57)));
}
