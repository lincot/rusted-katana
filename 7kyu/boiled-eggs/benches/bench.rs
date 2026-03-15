#![feature(test)]

extern crate test;
use boiled_eggs::cooking_time;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| cooking_time(black_box(10)));
}
