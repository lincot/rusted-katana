#![feature(test)]

extern crate test;
use difference_of_volumes_of_cuboids::find_difference;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_difference(black_box(&[1, 2, 3]), black_box(&[4, 5, 6])));
}
