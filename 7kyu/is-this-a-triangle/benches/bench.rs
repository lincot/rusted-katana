#![no_std]
#![feature(test)]

extern crate test;
use is_this_a_triangle::is_triangle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(is_triangle(black_box(5), black_box(1), black_box(5)));
        }
    });
}
