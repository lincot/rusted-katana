#![feature(test)]

extern crate test;
use euler_squares::create_euler_square;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| create_euler_square(black_box(33)));
}
