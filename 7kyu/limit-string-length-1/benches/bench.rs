#![no_std]
#![feature(test)]

extern crate test;
use limit_string_length_1::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("Тестовая строка"), black_box(9)));
}
