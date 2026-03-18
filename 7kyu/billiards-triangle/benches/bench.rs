#![feature(test)]

extern crate test;
use billiards_triangle::pyramid;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pyramid(black_box(9999)));
}
