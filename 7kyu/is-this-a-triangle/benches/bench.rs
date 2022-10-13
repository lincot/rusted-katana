#![no_std]
#![feature(test)]

extern crate test;
use is_this_a_triangle::is_triangle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(5);
    let b = black_box(1);
    let c = black_box(5);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(is_triangle(a, b, c));
        }
    });
}
