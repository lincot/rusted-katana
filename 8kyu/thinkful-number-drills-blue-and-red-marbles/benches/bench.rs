#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use thinkful_number_drills_blue_and_red_marbles::guess_blue;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| guess_blue(black_box(12), black_box(18), black_box(4), black_box(6)));
}
