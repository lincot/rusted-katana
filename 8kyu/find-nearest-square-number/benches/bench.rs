#![feature(test)]

extern crate test;
use find_nearest_square_number::nearest_sq;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| nearest_sq(black_box(786_737)));
}
