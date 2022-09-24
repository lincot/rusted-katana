#![no_std]
#![feature(test)]

extern crate test;
use dashatize_it::dashatize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(-28369);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(dashatize(n));
        }
    });
}
