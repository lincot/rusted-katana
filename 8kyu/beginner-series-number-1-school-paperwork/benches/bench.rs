#![no_std]
#![feature(test)]

extern crate test;
use beginner_series_number_1_school_paperwork::paperwork;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(30);
    let m = black_box(10);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(paperwork(n, m));
        }
    });
}
