#![no_std]
#![feature(test)]

extern crate test;
use deodorant_evaporator::evaporator;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let content = black_box(10.);
    let evap_per_day = black_box(10);
    let threshold = black_box(10);
    bencher.iter(|| evaporator(content, evap_per_day, threshold));
}
