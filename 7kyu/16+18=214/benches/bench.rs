#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const NUM1: u32 = 645612459;
const NUM2: u32 = 1837889123;

#[bench]
fn bench(bencher: &mut Bencher) {
    let num1 = black_box(NUM1);
    let num2 = black_box(NUM2);

    bencher.iter(|| solution::add(num1, num2))
}
