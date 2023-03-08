#![no_std]
#![feature(test)]

extern crate test;
use simple_fun_number_79_delete_a_digit::delete_digit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(delete_digit(black_box(653_424)));
        }
    });
}
