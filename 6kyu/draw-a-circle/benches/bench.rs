#![no_std]
#![feature(test)]

extern crate test;
use draw_a_circle::circle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| circle(black_box(if cfg!(miri) { 10 } else { 200 })));
}
