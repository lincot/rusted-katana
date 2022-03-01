#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let str = black_box("camel case method");
    bencher.iter(|| solution::camel_case(str))
}
