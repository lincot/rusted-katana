#![feature(test)]

extern crate test;
use over_the_road_1::over_the_road;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| over_the_road(black_box(23_633_656_673), black_box(310_027_696_726)));
}
