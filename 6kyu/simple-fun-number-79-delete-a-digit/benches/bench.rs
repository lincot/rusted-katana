#![feature(test)]

extern crate test;
use simple_fun_number_79_delete_a_digit::delete_digit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| delete_digit(black_box(653_424)));
}
