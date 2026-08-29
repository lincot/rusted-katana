#![feature(test)]

extern crate test;
use combining_ratios::merge_ratios;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| merge_ratios(black_box("6937:3221"), black_box("7:108")));
}
