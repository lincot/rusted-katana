#![feature(test)]

extern crate test;
use parabolic_arc_length::len_curve;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| len_curve(black_box(1000)));
}
