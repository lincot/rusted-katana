#![no_std]
#![feature(test)]

extern crate test;
use draw_a_circle::circle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let radius = black_box(200);
    bencher.iter(|| circle(radius));
}
