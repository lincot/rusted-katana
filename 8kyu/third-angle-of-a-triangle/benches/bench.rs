#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use third_angle_of_a_triangle::other_angle;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| other_angle(black_box(90), black_box(30)));
}
