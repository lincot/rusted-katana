#![feature(test)]

extern crate test;
use financing_plan_on_planet_xy140z_n::finance;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| finance(black_box(100)));
}
