#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box(896_219_342);
    bencher.iter(|| solution::max_rot(num));
}
