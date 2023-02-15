#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use triangle_number_check::is_triangle_number;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(is_triangle_number(black_box(25_673_348)));
        }
    });
}
