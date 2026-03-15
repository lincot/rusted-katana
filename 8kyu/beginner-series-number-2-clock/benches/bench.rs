#![feature(test)]

extern crate test;
use beginner_series_number_2_clock::past;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| past(black_box(5), black_box(30), black_box(20)));
}
