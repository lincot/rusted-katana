#![feature(test)]

extern crate test;
use calculate_pyramid_height::pyramid_height;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pyramid_height(black_box(9999)));
}
