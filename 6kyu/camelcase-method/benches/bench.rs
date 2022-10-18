#![no_std]
#![feature(test)]

extern crate test;
use camelcase_method::camel_case;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| camel_case(black_box("верблюд случай метод")));
}
