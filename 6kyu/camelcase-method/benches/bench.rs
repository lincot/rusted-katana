#![feature(test)]

extern crate test;
use camelcase_method::camel_case;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let str = black_box("camel case method");
    bencher.iter(|| camel_case(str));
}
