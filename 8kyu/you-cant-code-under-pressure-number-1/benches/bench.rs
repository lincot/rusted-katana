#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use you_cant_code_under_pressure_number_1::double_integer;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| double_integer(black_box(25)));
}
