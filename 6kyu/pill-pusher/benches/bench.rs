#![no_std]
#![feature(test)]

extern crate test;
use pill_pusher::prescribe;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| prescribe(black_box(6077), black_box(24), black_box(33)));
}
