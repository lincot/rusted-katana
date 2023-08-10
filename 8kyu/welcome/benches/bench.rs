#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use welcome::greet;

#[bench]
fn bench_estonian(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(greet(black_box("estonian")));
        }
    });
}

#[bench]
fn bench_swedish(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(greet(black_box("swedish")));
        }
    });
}
