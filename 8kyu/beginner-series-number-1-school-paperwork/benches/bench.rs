#![feature(test)]

extern crate test;
use beginner_series_number_1_school_paperwork::paperwork;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| paperwork(black_box(30), black_box(10)));
}
