#![feature(test)]

extern crate test;
use camelcase_method::camel_case;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| camel_case(black_box("camel case method")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| camel_case(black_box("верблюд случай метод")));
}
