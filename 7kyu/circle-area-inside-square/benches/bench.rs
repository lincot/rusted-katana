#![feature(test)]

extern crate test;
use circle_area_inside_square::square_area_to_circle;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| square_area_to_circle(black_box(20.)));
}
