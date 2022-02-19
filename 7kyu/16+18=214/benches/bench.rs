#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let num1 = black_box(645612459);
    let num2 = black_box(1837889123);
    bencher.iter(|| solution::add(num1, num2))
}
