#![no_std]
#![feature(test)]

extern crate test;
use magic_triangle_solutions::magic_triangle_solutions;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| magic_triangle_solutions(black_box(&[0, 4, 0, 0, 2, 0, 0, 5, 0])));
}
