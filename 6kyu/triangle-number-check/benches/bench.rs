#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use triangle_number_check::is_triangle_number;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| is_triangle_number(black_box(25_673_348)));
}
