#![no_std]
#![feature(test)]

extern crate test;
use pandemia::infected;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("01000000X000X011X0X");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(infected(s));
        }
    });
}
