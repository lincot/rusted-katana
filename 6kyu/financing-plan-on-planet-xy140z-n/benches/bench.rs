#![no_std]
#![feature(test)]

extern crate test;
use financing_plan_on_planet_xy140z_n::finance;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(finance(black_box(100)));
        }
    });
}
