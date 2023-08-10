#![no_std]
#![feature(test)]

extern crate test;
use simple_fun_number_2_circle_of_numbers::circle_of_numbers;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(circle_of_numbers(black_box(300), black_box(200)));
        }
    });
}
